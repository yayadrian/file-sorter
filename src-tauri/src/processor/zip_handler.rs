use super::*;
use crate::processor::image_converter::{ConversionResult, ImageConverter};
use crate::processor::temp_manager::TempManager;
use crate::report::ReportBuilder;
use crate::utils::collision::CollisionManager;
use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use tauri::AppHandle;
use zip::write::FileOptions;
use zip::ZipArchive;

pub async fn process_zip_file(
    app: &AppHandle,
    state: &ProcessorState,
    job: &JobInfo,
) -> Result<String> {
    let input_path = Path::new(&job.input_path);
    
    // Create temp directory for this job
    let temp_manager = TempManager::new(&job.id)?;
    let extract_dir = temp_manager.get_extract_dir()?;
    let staging_dir = temp_manager.get_staging_dir()?;

    // Open input zip
    let input_file = File::open(input_path)
        .context("Failed to open input zip file")?;
    let mut archive = ZipArchive::new(input_file)
        .context("Failed to read zip archive")?;

    // Scan phase
    state.emit_progress(
        app,
        &job.id,
        ProgressInfo {
            current_file: 0,
            total_files: archive.len(),
            current_filename: "Scanning...".to_string(),
            phase: ProcessingPhase::Scanning,
        },
    );

    // Check for cancellation
    if state.cancel_flag.load(Ordering::SeqCst) {
        return Err(anyhow::anyhow!("Processing cancelled"));
    }

    // Build list of image files to process
    let mut image_entries = Vec::new();
    let converter = ImageConverter::new();
    let mut report = ReportBuilder::new(input_path);

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let file_name = file.name().to_string();
        
        report.increment_scanned();

        // Skip directories
        if file.is_dir() {
            continue;
        }

        // Skip nested zips
        if file_name.to_lowercase().ends_with(".zip") {
            report.add_skipped(
                file_name.clone(),
                "Nested zip files are ignored".to_string(),
            );
            continue;
        }

        // Check if it's an image we should process
        let path = Path::new(&file_name);
        if converter.should_process(path) {
            image_entries.push((i, file_name));
        }
    }

    let total_images = image_entries.len();
    if total_images == 0 {
        return Err(anyhow::anyhow!("No image files found in zip"));
    }

    // Processing phase
    let mut collision_manager = CollisionManager::new();
    let mut processed_files: Vec<(PathBuf, PathBuf)> = Vec::new(); // (staging_path, zip_path)

    for (idx, (zip_index, file_name)) in image_entries.iter().enumerate() {
        // Check for cancellation
        if state.cancel_flag.load(Ordering::SeqCst) {
            return Err(anyhow::anyhow!("Processing cancelled"));
        }

        state.emit_progress(
            app,
            &job.id,
            ProgressInfo {
                current_file: idx + 1,
                total_files: total_images,
                current_filename: file_name.clone(),
                phase: ProcessingPhase::Converting,
            },
        );

        // Extract file to temp
        let mut zip_file = archive.by_index(*zip_index)?;
        let zip_uncompressed_size = zip_file.size();
        let zip_compressed_size = zip_file.compressed_size();
        let extract_path = extract_dir.join(file_name);
        
        if let Some(parent) = extract_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut extracted_file = File::create(&extract_path)?;
        std::io::copy(&mut zip_file, &mut extracted_file)?;
        drop(extracted_file);

        // Determine output path and handle collisions
        let original_path = Path::new(file_name);
        let mut output_relative_path = original_path.to_path_buf();

        // If converting, change extension to .jpg
        let needs_conversion = !matches!(
            original_path.extension().and_then(|e| e.to_str()),
            Some("jpg") | Some("jpeg") | Some("png") | Some("gif")
        );

        if needs_conversion {
            output_relative_path = change_extension(&output_relative_path, "jpg");
        }

        // Get unique path (handles collisions)
        let unique_relative_path = collision_manager.get_unique_path(&output_relative_path);
        let staging_path = staging_dir.join(&unique_relative_path);

        // Process the image
        match converter.process_image(&extract_path, &staging_path) {
            Ok(ConversionResult::Copied) => {
                report.add_copied(
                    file_name.clone(),
                    unique_relative_path.to_string_lossy().to_string(),
                );
            }
            Ok(ConversionResult::Converted { original_format }) => {
                let metadata_preserved = crate::utils::metadata::MetadataHandler::format_has_exif(&original_format);
                report.add_conversion(
                    file_name.clone(),
                    unique_relative_path.to_string_lossy().to_string(),
                    original_format,
                    metadata_preserved,
                );
            }
            Err(e) => {
                let extracted_size = fs::metadata(&extract_path).map(|m| m.len()).ok();
                let extension = original_path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("");

                eprintln!("Error processing image from zip");
                eprintln!("  file_name: {}", file_name);
                eprintln!("  extension: {}", extension);
                eprintln!("  needs_conversion: {}", needs_conversion);
                eprintln!("  extract_path: {}", extract_path.display());
                eprintln!("  staging_path: {}", staging_path.display());
                eprintln!("  zip_uncompressed_size: {} bytes", zip_uncompressed_size);
                eprintln!("  zip_compressed_size: {} bytes", zip_compressed_size);
                if let Some(size) = extracted_size {
                    eprintln!("  extracted_size: {} bytes", size);
                } else {
                    eprintln!("  extracted_size: <unavailable>");
                }
                eprintln!("  error: {:#}", e);

                // Fail-fast: abort on any conversion error
                return Err(e.context(format!("Failed to process image: {}", file_name)));
            }
        }

        processed_files.push((staging_path, unique_relative_path));
    }

    // Packaging phase
    state.emit_progress(
        app,
        &job.id,
        ProgressInfo {
            current_file: total_images,
            total_files: total_images,
            current_filename: "Creating output zip...".to_string(),
            phase: ProcessingPhase::Packaging,
        },
    );

    // Check for cancellation one more time
    if state.cancel_flag.load(Ordering::SeqCst) {
        return Err(anyhow::anyhow!("Processing cancelled"));
    }

    // Create output zip in temp location
    let temp_output_path = temp_manager.get_output_zip_path();
    let output_file = File::create(&temp_output_path)?;
    let mut zip_writer = zip::ZipWriter::new(output_file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // Add all processed files to zip
    for (staging_path, zip_path) in processed_files {
        let mut file = File::open(&staging_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        zip_writer.start_file(zip_path.to_string_lossy(), options)?;
        zip_writer.write_all(&buffer)?;
    }

    // Add report.json to root of zip
    let report_json = report.to_json()?;
    zip_writer.start_file("report.json", options)?;
    zip_writer.write_all(report_json.as_bytes())?;

    zip_writer.finish()?;

    // Move output zip to Downloads folder
    let downloads_dir = dirs::download_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find Downloads folder"))?;

    let input_filename = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    
    let mut output_filename = format!("{}-converted.zip", input_filename);
    let mut final_output_path = downloads_dir.join(&output_filename);

    // Handle collisions in Downloads folder
    let mut counter = 1;
    while final_output_path.exists() {
        output_filename = format!("{}-converted-{}.zip", input_filename, counter);
        final_output_path = downloads_dir.join(&output_filename);
        counter += 1;
    }

    fs::copy(&temp_output_path, &final_output_path)
        .context("Failed to copy output zip to Downloads")?;

    Ok(final_output_path.to_string_lossy().to_string())
}

fn change_extension(path: &Path, new_ext: &str) -> PathBuf {
    let mut result = path.to_path_buf();
    result.set_extension(new_ext);
    result
}
