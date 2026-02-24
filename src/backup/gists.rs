use crate::{config::BackupConfig, error::Result};

pub async fn backup_gists(_config: &BackupConfig) -> Result<()> {
    Ok(())
}
