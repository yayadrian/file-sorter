use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct TempManager {
    temp_dir: PathBuf,
}

impl TempManager {
    pub fn new(job_id: &str) -> Result<Self> {
        let temp_dir = std::env::temp_dir().join(format!("file-sorter-{}", job_id));
        fs::create_dir_all(&temp_dir)
            .context("Failed to create temp directory")?;

        Ok(Self { temp_dir })
    }

    pub fn get_path(&self) -> &Path {
        &self.temp_dir
    }

    pub fn create_subdir(&self, name: &str) -> Result<PathBuf> {
        let subdir = self.temp_dir.join(name);
        fs::create_dir_all(&subdir)
            .context(format!("Failed to create subdirectory: {}", name))?;
        Ok(subdir)
    }

    pub fn get_extract_dir(&self) -> Result<PathBuf> {
        self.create_subdir("extract")
    }

    pub fn get_staging_dir(&self) -> Result<PathBuf> {
        self.create_subdir("staging")
    }

    pub fn get_output_zip_path(&self) -> PathBuf {
        self.temp_dir.join("output.zip")
    }
}

impl Drop for TempManager {
    fn drop(&mut self) {
        // Clean up temp directory when TempManager is dropped
        let _ = fs::remove_dir_all(&self.temp_dir);
    }
}
