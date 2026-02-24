use crate::{config::BackupConfig, error::Result};

pub async fn backup_pulls(_config: &BackupConfig) -> Result<()> {
    Ok(())
}
