use crate::{config::BackupConfig, error::Result};

pub async fn backup_wiki(_config: &BackupConfig) -> Result<()> {
    Ok(())
}
