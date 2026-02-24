pub mod repositories;

use tracing::info;

use crate::{config::BackupConfig, error::Result};

#[derive(Debug, Clone)]
pub struct BackupOrchestrator {
    config: BackupConfig,
}

impl BackupOrchestrator {
    pub fn new(config: BackupConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<()> {
        info!("backup orchestration started");
        repositories::backup_repositories(&self.config).await
    }
}
