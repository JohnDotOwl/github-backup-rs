use tracing::info;

use crate::{backup::BackupOrchestrator, config::BackupConfig, error::Result};

use super::args::CliArgs;

pub async fn run_cli(args: CliArgs) -> Result<()> {
    let config = BackupConfig::from_cli(&args)?;
    let start_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    info!("[{}] starting backup run from CLI", start_time);
    BackupOrchestrator::new(config).run().await
}
