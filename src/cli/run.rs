use tracing::info;

use crate::{backup::BackupOrchestrator, config::BackupConfig, error::Result};

use super::args::CliArgs;

pub async fn run_cli(args: CliArgs) -> Result<()> {
    let config = BackupConfig::from_cli(&args)?;
    info!("starting backup run from CLI");
    BackupOrchestrator::new(config).run().await
}
