use clap::Parser;
use github_backup_rs::{cli::args::CliArgs, cli::run::run_cli, plugin::rpc::run_plugin_stdio};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let args = CliArgs::parse();
    if args.plugin {
        run_plugin_stdio().await?;
    } else {
        run_cli(args).await?;
    }

    Ok(())
}
