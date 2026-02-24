use crate::{config::BackupConfig, error::Result};

pub async fn backup_milestones(_config: &BackupConfig) -> Result<()> {
    Ok(())
}
