use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(
    name = "github-backup",
    about = "Clone and update GitHub repositories for a user or organization",
    version
)]
pub struct CliArgs {
    #[arg(value_name = "USER_OR_ORG")]
    pub target: Option<String>,

    #[arg(short = 'o', long = "output", default_value = ".")]
    pub output_dir: PathBuf,

    #[arg(long)]
    pub organization: bool,

    #[arg(long, env = "GITHUB_TOKEN")]
    pub token: Option<String>,

    #[arg(long = "token-file")]
    pub token_file: Option<PathBuf>,

    #[arg(long)]
    pub use_keychain: bool,

    #[arg(long)]
    pub keychain_service: Option<String>,

    #[arg(long = "repo", value_name = "OWNER/REPO")]
    pub repositories: Vec<String>,

    #[arg(long, default_value_t = 4)]
    pub concurrency: usize,

    #[arg(long, default_value_t = 5)]
    pub max_retries: u32,

    #[arg(long, default_value_t = 30)]
    pub request_timeout_seconds: u64,

    #[arg(long)]
    pub api_base_url: Option<String>,
}
