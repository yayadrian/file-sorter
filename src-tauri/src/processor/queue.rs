use super::*;
use crate::processor::zip_handler::process_zip_file;
use std::sync::atomic::Ordering;
use tauri::AppHandle;

pub async fn start_queue_processor(app: AppHandle, state: Arc<ProcessorState>) {
    // Prevent multiple processors from running
    if state.processing.swap(true, Ordering::SeqCst) {
        return;
    }

    tokio::spawn(async move {
        loop {
            // Reset cancel flag for next job
            state.cancel_flag.store(false, Ordering::SeqCst);

            // Get next pending job
            let job = match state.get_next_pending() {
                Some(j) => j,
                None => {
                    // No more jobs, stop processing
                    state.processing.store(false, Ordering::SeqCst);
                    break;
                }
            };

            // Process the job
            match process_zip_file(&app, &state, &job).await {
                Ok(output_path) => {
                    if state.cancel_flag.load(Ordering::SeqCst) {
                        state.mark_cancelled(&job.id);
                    } else {
                        state.mark_success(&app, &job.id, output_path);
                    }
                }
                Err(e) => {
                    if state.cancel_flag.load(Ordering::SeqCst) {
                        state.mark_cancelled(&job.id);
                    } else {
                        state.mark_failed(&app, &job.id, e.to_string());
                    }
                }
            }
        }
    });
}
