use crate::processor::ProcessorState;
use std::sync::Arc;

impl Clone for ProcessorState {
    fn clone(&self) -> Self {
        Self {
            jobs: Arc::clone(&self.jobs),
            cancel_flag: Arc::clone(&self.cancel_flag),
            processing: Arc::clone(&self.processing),
        }
    }
}
