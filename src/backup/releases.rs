use crate::{config::BackupConfig, error::Result};

pub async fn backup_releases(_config: &BackupConfig) -> Result<()> {
    Ok(())
}
