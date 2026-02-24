use crate::{config::BackupConfig, error::Result};

pub async fn backup_attachments(_config: &BackupConfig) -> Result<()> {
    Ok(())
}
