use crate::{config::BackupConfig, error::Result};

pub async fn backup_security(_config: &BackupConfig) -> Result<()> {
    Ok(())
}
