use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncrementalState {
    pub last_successful_sync_epoch_seconds: Option<u64>,
}

impl IncrementalState {
    pub fn should_refresh(&self, updated_at_epoch_seconds: Option<u64>) -> bool {
        match (
            self.last_successful_sync_epoch_seconds,
            updated_at_epoch_seconds,
        ) {
            (None, _) => true,
            (Some(_), None) => true,
            (Some(last_sync), Some(updated)) => updated > last_sync,
        }
    }

    pub fn mark_synced(&mut self, epoch_seconds: u64) {
        self.last_successful_sync_epoch_seconds = Some(epoch_seconds);
    }
}
