use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::Path;

const APP_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessingReport {
    pub app_version: String,
    pub timestamp: String,
    pub input_zip: String,
    pub stats: ReportStats,
    pub conversions: Vec<ConversionRecord>,
    pub skipped: Vec<SkippedRecord>,
    pub metadata_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportStats {
    pub files_scanned: usize,
    pub files_included: usize,
    pub files_converted: usize,
    pub files_skipped: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionRecord {
    pub original_path: String,
    pub output_path: String,
    pub original_format: String,
    pub metadata_preserved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkippedRecord {
    pub path: String,
    pub reason: String,
}

pub struct ReportBuilder {
    input_zip_name: String,
    conversions: Vec<ConversionRecord>,
    skipped: Vec<SkippedRecord>,
    files_scanned: usize,
    files_included: usize,
    files_converted: usize,
}

impl ReportBuilder {
    pub fn new(input_zip_path: &Path) -> Self {
        let input_zip_name = input_zip_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown.zip")
            .to_string();

        Self {
            input_zip_name,
            conversions: Vec::new(),
            skipped: Vec::new(),
            files_scanned: 0,
            files_included: 0,
            files_converted: 0,
        }
    }

    pub fn increment_scanned(&mut self) {
        self.files_scanned += 1;
    }

    pub fn add_conversion(
        &mut self,
        original_path: String,
        output_path: String,
        original_format: String,
        metadata_preserved: bool,
    ) {
        self.conversions.push(ConversionRecord {
            original_path,
            output_path,
            original_format,
            metadata_preserved,
        });
        self.files_included += 1;
        self.files_converted += 1;
    }

    pub fn add_copied(&mut self, original_path: String, output_path: String) {
        self.files_included += 1;
        // We don't add to conversions list since it was just copied
    }

    pub fn add_skipped(&mut self, path: String, reason: String) {
        self.skipped.push(SkippedRecord { path, reason });
    }

    pub fn build(self) -> ProcessingReport {
        let mut metadata_notes = Vec::new();
        
        // Add notes about formats that were converted
        let mut formats_seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        for conv in &self.conversions {
            formats_seen.insert(conv.original_format.clone());
        }

        for format in formats_seen {
            let note = crate::utils::metadata::MetadataHandler::get_preservation_note(&format);
            metadata_notes.push(note);
        }

        // Add general note about JPEG quality
        if self.files_converted > 0 {
            metadata_notes.push("All converted images encoded as JPEG with quality 95".to_string());
        }

        ProcessingReport {
            app_version: APP_VERSION.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            input_zip: self.input_zip_name,
            stats: ReportStats {
                files_scanned: self.files_scanned,
                files_included: self.files_included,
                files_converted: self.files_converted,
                files_skipped: self.skipped.len(),
            },
            conversions: self.conversions,
            skipped: self.skipped,
            metadata_notes,
        }
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        let report = self.clone().build();
        serde_json::to_string_pretty(&report).map_err(|e| anyhow::anyhow!("Failed to serialize report: {}", e))
    }
}

impl Clone for ReportBuilder {
    fn clone(&self) -> Self {
        Self {
            input_zip_name: self.input_zip_name.clone(),
            conversions: self.conversions.clone(),
            skipped: self.skipped.clone(),
            files_scanned: self.files_scanned,
            files_included: self.files_included,
            files_converted: self.files_converted,
        }
    }
}
