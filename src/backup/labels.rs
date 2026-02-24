use crate::{config::BackupConfig, error::Result};

pub async fn backup_labels(_config: &BackupConfig) -> Result<()> {
    Ok(())
}
