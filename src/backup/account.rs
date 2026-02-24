use crate::{config::BackupConfig, error::Result};

pub async fn backup_account(_config: &BackupConfig) -> Result<()> {
    Ok(())
}
