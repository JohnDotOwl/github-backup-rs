use crate::{config::BackupConfig, error::Result};

pub async fn backup_hooks(_config: &BackupConfig) -> Result<()> {
    Ok(())
}
