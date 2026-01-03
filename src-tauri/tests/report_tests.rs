// Tests for report.json generation

#[cfg(test)]
mod report_tests {
    use file_sorter_lib::report::{ReportBuilder, ProcessingReport};
    use std::path::Path;

    #[test]
    fn test_report_builder_creation() {
        let input_path = Path::new("/path/to/test.zip");
        let builder = ReportBuilder::new(input_path);
        let report = builder.build();

        assert_eq!(report.input_zip, "test.zip");
        assert_eq!(report.app_version, "1.0.0");
        assert!(!report.timestamp.is_empty());
    }

    #[test]
    fn test_report_stats() {
        let input_path = Path::new("/path/to/test.zip");
        let mut builder = ReportBuilder::new(input_path);

        builder.increment_scanned();
        builder.increment_scanned();
        builder.increment_scanned();

        builder.add_conversion(
            "image1.heic".to_string(),
            "image1.jpg".to_string(),
            "HEIC".to_string(),
            true,
        );

        builder.add_copied("image2.jpg".to_string(), "image2.jpg".to_string());

        builder.add_skipped("nested.zip".to_string(), "Nested zip ignored".to_string());

        let report = builder.build();

        assert_eq!(report.stats.files_scanned, 3);
        assert_eq!(report.stats.files_included, 2);
        assert_eq!(report.stats.files_converted, 1);
        assert_eq!(report.stats.files_skipped, 1);

        assert_eq!(report.conversions.len(), 1);
        assert_eq!(report.skipped.len(), 1);
    }

    #[test]
    fn test_report_json_serialization() {
        let input_path = Path::new("/path/to/test.zip");
        let mut builder = ReportBuilder::new(input_path);

        builder.increment_scanned();
        builder.add_conversion(
            "test.heic".to_string(),
            "test.jpg".to_string(),
            "HEIC".to_string(),
            true,
        );

        let json = builder.to_json().unwrap();
        
        assert!(json.contains("appVersion"));
        assert!(json.contains("timestamp"));
        assert!(json.contains("test.zip"));
        assert!(json.contains("HEIC"));
    }

    #[test]
    fn test_metadata_notes() {
        let input_path = Path::new("/path/to/test.zip");
        let mut builder = ReportBuilder::new(input_path);

        builder.add_conversion(
            "test.heic".to_string(),
            "test.jpg".to_string(),
            "HEIC".to_string(),
            true,
        );

        let report = builder.build();

        assert!(!report.metadata_notes.is_empty());
        assert!(report.metadata_notes.iter().any(|note| note.contains("quality 95")));
    }

    #[test]
    fn test_multiple_format_conversions() {
        let input_path = Path::new("/path/to/test.zip");
        let mut builder = ReportBuilder::new(input_path);

        builder.add_conversion("img1.heic".to_string(), "img1.jpg".to_string(), "HEIC".to_string(), true);
        builder.add_conversion("img2.webp".to_string(), "img2.jpg".to_string(), "WEBP".to_string(), true);
        builder.add_conversion("img3.bmp".to_string(), "img3.jpg".to_string(), "BMP".to_string(), false);

        let report = builder.build();

        assert_eq!(report.conversions.len(), 3);
        assert!(report.conversions.iter().any(|c| c.original_format == "HEIC"));
        assert!(report.conversions.iter().any(|c| c.original_format == "WEBP"));
        assert!(report.conversions.iter().any(|c| c.original_format == "BMP"));
    }
}
