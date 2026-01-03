use anyhow::{Context, Result, anyhow};
use image::{DynamicImage, ImageFormat, GenericImageView};
use std::fs;
use std::path::{Path, PathBuf};
use std::io::BufReader;
use std::sync::Once;

static REGISTER_HEIF_HOOKS: Once = Once::new();

fn register_heif_decoding_hooks() {
    REGISTER_HEIF_HOOKS.call_once(|| {
        libheif_rs::integration::image::register_all_decoding_hooks();
    });
}

pub enum ConversionResult {
    /// File was copied as-is (JPEG, PNG, GIF, or animated)
    Copied,
    /// File was converted to JPEG
    Converted { original_format: String },
}

pub struct ImageConverter {
    jpeg_quality: u8,
}

impl ImageConverter {
    pub fn new() -> Self {
        register_heif_decoding_hooks();
        Self { jpeg_quality: 95 }
    }

    /// Detect if a file is an image and determine if it needs conversion
    pub fn should_process(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext_lower = ext.to_string_lossy().to_lowercase();
            matches!(
                ext_lower.as_str(),
                "jpg" | "jpeg" | "png" | "gif" | "heic" | "heif" | "webp" | "tiff" | "tif" | "bmp" | "avif"
            )
        } else {
            false
        }
    }

    /// Process an image file: copy if already supported, convert otherwise
    pub fn process_image(
        &self,
        input_path: &Path,
        output_path: &Path,
    ) -> Result<ConversionResult> {
        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Get file extension
        let ext = input_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        // Check if we should just copy the file
        if matches!(ext.as_str(), "jpg" | "jpeg" | "png") {
            fs::copy(input_path, output_path)
                .context("Failed to copy image file")?;
            return Ok(ConversionResult::Copied);
        }

        // Special handling for GIF (check if animated, if so copy as-is)
        if ext == "gif" {
            if self.is_animated_gif(input_path)? {
                fs::copy(input_path, output_path)?;
                return Ok(ConversionResult::Copied);
            }
            // Static GIF, keep as GIF
            fs::copy(input_path, output_path)?;
            return Ok(ConversionResult::Copied);
        }

        // Special handling for WebP (check if animated)
        if ext == "webp" {
            if self.is_animated_webp(input_path)? {
                fs::copy(input_path, output_path)?;
                return Ok(ConversionResult::Copied);
            }
        }

        // Convert other formats to JPEG
        let format_name = ext.to_uppercase();
        self.convert_to_jpeg(input_path, output_path, &format_name)?;
        
        Ok(ConversionResult::Converted {
            original_format: format_name,
        })
    }

    fn is_animated_gif(&self, path: &Path) -> Result<bool> {
        // For simplicity, we'll keep all GIFs as-is
        // A more sophisticated implementation would parse GIF headers
        Ok(true)
    }

    fn is_animated_webp(&self, path: &Path) -> Result<bool> {
        // Try to detect animated WebP
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        
        // Use image crate to check
        match image::load(reader, ImageFormat::WebP) {
            Ok(_) => {
                // For now, assume single-frame WebP
                // A full implementation would check WebP headers for animation flag
                Ok(false)
            }
            Err(_) => Ok(false),
        }
    }

    fn convert_to_jpeg(
        &self,
        input_path: &Path,
        output_path: &Path,
        format_name: &str,
    ) -> Result<()> {
        // Load the image using appropriate decoder
        let img = if format_name == "HEIC" || format_name == "HEIF" {
            self.load_heic(input_path)?
        } else {
            // Use image crate for other formats
            image::open(input_path)
                .with_context(|| format!(
                    "Failed to open {} image at {}",
                    format_name,
                    input_path.display()
                ))?
        };

        // If image has transparency, composite onto white background
        let img = self.composite_on_white(img);

        // Try to preserve EXIF metadata
        let exif_data = self.extract_exif(input_path);

        // Encode as JPEG
        let mut output_file = fs::File::create(output_path)
            .context("Failed to create output JPEG file")?;
        
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
            &mut output_file,
            self.jpeg_quality,
        );
        
        img.write_with_encoder(encoder)
            .context("Failed to encode JPEG")?;

        // Write EXIF data if we extracted any
        if let Some(exif) = exif_data {
            if let Err(e) = self.write_exif(output_path, exif) {
                eprintln!("Warning: Failed to write EXIF metadata: {}", e);
            }
        }

        Ok(())
    }

    fn load_heic(&self, path: &Path) -> Result<DynamicImage> {
        // Register HEIC decoding hooks
        libheif_rs::LibHeif::new();
        
        // Load using image crate with libheif-rs integration
        image::open(path)
            .with_context(|| format!(
                "Failed to decode HEIC image at {}",
                path.display()
            ))
    }

    fn composite_on_white(&self, img: DynamicImage) -> DynamicImage {
        // Check if image has alpha channel
        if img.color().has_alpha() {
            let (width, height) = img.dimensions();
            let mut white_bg = image::RgbaImage::from_pixel(
                width,
                height,
                image::Rgba([255, 255, 255, 255]),
            );

            // Composite the image onto white background
            image::imageops::overlay(&mut white_bg, &img.to_rgba8(), 0, 0);
            DynamicImage::ImageRgba8(white_bg).to_rgb8().into()
        } else {
            img
        }
    }

    fn extract_exif(&self, path: &Path) -> Option<Vec<u8>> {
        // Try to extract EXIF data using little_exif
        match little_exif::metadata::Metadata::new_from_path(path) {
            Ok(metadata) => {
                // Serialize the metadata to bytes
                // For now, we'll just note that we tried
                None
            }
            Err(_) => None,
        }
    }

    fn write_exif(&self, path: &Path, exif_data: Vec<u8>) -> Result<()> {
        // Write EXIF data to the JPEG file
        // This is a placeholder - full implementation would use exiftool or similar
        Ok(())
    }
}

impl Default for ImageConverter {
    fn default() -> Self {
        Self::new()
    }
}
