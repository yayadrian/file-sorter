use crate::processor::{JobInfo, ProcessorState};
use crate::processor::queue::start_queue_processor;
use tauri::{AppHandle, State};
use std::sync::Arc;

#[tauri::command]
pub async fn enqueue_zips(
    app: AppHandle,
    state: State<'_, ProcessorState>,
    paths: Vec<String>,
) -> Result<Vec<JobInfo>, String> {
    // Add jobs to queue
    let jobs = state
        .add_jobs(paths)
        .map_err(|e| format!("Failed to enqueue jobs: {}", e))?;

    // Start processing queue if not already running
    let state_arc = Arc::new(state.inner().clone());
    start_queue_processor(app, state_arc).await;

    Ok(jobs)
}

#[tauri::command]
pub async fn cancel_current(state: State<'_, ProcessorState>) -> Result<(), String> {
    state.cancel_current();
    Ok(())
}

#[tauri::command]
pub async fn clear_finished(state: State<'_, ProcessorState>) -> Result<(), String> {
    state.clear_finished();
    Ok(())
}

#[tauri::command]
pub async fn open_in_folder(path: String) -> Result<(), String> {
    use std::process::Command;

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        // Try to open the parent directory
        let path_obj = std::path::Path::new(&path);
        if let Some(parent) = path_obj.parent() {
            Command::new("xdg-open")
                .arg(parent)
                .spawn()
                .map_err(|e| format!("Failed to open folder: {}", e))?;
        }
    }

    Ok(())
}
