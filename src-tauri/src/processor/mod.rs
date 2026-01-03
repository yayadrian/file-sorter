pub mod queue;
pub mod zip_handler;
pub mod image_converter;
pub mod temp_manager;
mod state_impl;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobInfo {
    pub id: String,
    pub input_path: String,
    pub status: JobStatus,
    pub progress: Option<ProgressInfo>,
    pub output_path: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Processing,
    Success,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressInfo {
    pub current_file: usize,
    pub total_files: usize,
    pub current_filename: String,
    pub phase: ProcessingPhase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessingPhase {
    Scanning,
    Converting,
    Packaging,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessingStats {
    pub files_scanned: usize,
    pub files_included: usize,
    pub files_converted: usize,
    pub files_skipped: usize,
}

pub struct ProcessorState {
    pub jobs: Arc<Mutex<Vec<JobInfo>>>,
    pub cancel_flag: Arc<AtomicBool>,
    pub processing: Arc<AtomicBool>,
}

impl ProcessorState {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(Vec::new())),
            cancel_flag: Arc::new(AtomicBool::new(false)),
            processing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn add_jobs(&self, paths: Vec<String>) -> Result<Vec<JobInfo>> {
        let mut jobs = self.jobs.lock().unwrap();
        let new_jobs: Vec<JobInfo> = paths
            .into_iter()
            .map(|path| JobInfo {
                id: Uuid::new_v4().to_string(),
                input_path: path,
                status: JobStatus::Pending,
                progress: None,
                output_path: None,
                error: None,
            })
            .collect();

        jobs.extend(new_jobs.clone());
        Ok(new_jobs)
    }

    pub fn cancel_current(&self) {
        self.cancel_flag.store(true, Ordering::SeqCst);
    }

    pub fn clear_finished(&self) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.retain(|job| {
            job.status == JobStatus::Pending || job.status == JobStatus::Processing
        });
    }

    pub fn get_next_pending(&self) -> Option<JobInfo> {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.iter_mut()
            .find(|job| job.status == JobStatus::Pending)
            .map(|job| {
                job.status = JobStatus::Processing;
                job.clone()
            })
    }

    pub fn update_job(&self, id: &str, update: impl FnOnce(&mut JobInfo)) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.iter_mut().find(|j| j.id == id) {
            update(job);
        }
    }

    pub fn emit_progress(&self, app: &AppHandle, job_id: &str, progress: ProgressInfo) {
        self.update_job(job_id, |job| {
            job.progress = Some(progress.clone());
        });
        let _ = app.emit("processing-progress", progress);
    }

    pub fn mark_success(&self, app: &AppHandle, job_id: &str, output_path: String) {
        self.update_job(job_id, |job| {
            job.status = JobStatus::Success;
            job.output_path = Some(output_path.clone());
            job.progress = None;
        });
        let _ = app.emit("job-complete", serde_json::json!({
            "jobId": job_id,
            "outputPath": output_path,
        }));
    }

    pub fn mark_failed(&self, app: &AppHandle, job_id: &str, error: String) {
        self.update_job(job_id, |job| {
            job.status = JobStatus::Failed;
            job.error = Some(error.clone());
            job.progress = None;
        });
        let _ = app.emit("job-failed", serde_json::json!({
            "jobId": job_id,
            "error": error,
        }));
    }

    pub fn mark_cancelled(&self, job_id: &str) {
        self.update_job(job_id, |job| {
            job.status = JobStatus::Cancelled;
            job.progress = None;
        });
    }
}

impl Default for ProcessorState {
    fn default() -> Self {
        Self::new()
    }
}
