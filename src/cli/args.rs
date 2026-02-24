use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(
    name = "github-backup",
    about = "Backup GitHub repositories and metadata",
    version
)]
pub struct CliArgs {
    #[arg(value_name = "USER_OR_ORG")]
    pub target: Option<String>,

    #[arg(short = 'o', long = "output", default_value = ".")]
    pub output_dir: PathBuf,

    #[arg(long)]
    pub plugin: bool,

    #[arg(long)]
    pub organization: bool,

    #[arg(long)]
    pub token: Option<String>,

    #[arg(long = "token-file")]
    pub token_file: Option<PathBuf>,

    #[arg(long)]
    pub use_keychain: bool,

    #[arg(long)]
    pub keychain_service: Option<String>,

    #[arg(long = "repo", value_name = "OWNER/REPO")]
    pub repositories: Vec<String>,

    #[arg(long = "repositories")]
    pub include_repositories: bool,

    #[arg(long = "issues")]
    pub include_issues: bool,

    #[arg(long = "pulls")]
    pub include_pulls: bool,

    #[arg(long = "releases")]
    pub include_releases: bool,

    #[arg(long = "labels")]
    pub include_labels: bool,

    #[arg(long = "milestones")]
    pub include_milestones: bool,

    #[arg(long = "hooks")]
    pub include_hooks: bool,

    #[arg(long = "security")]
    pub include_security: bool,

    #[arg(long = "wiki")]
    pub include_wiki: bool,

    #[arg(long = "gists")]
    pub include_gists: bool,

    #[arg(long = "account")]
    pub include_account: bool,

    #[arg(long = "attachments")]
    pub include_attachments: bool,

    #[arg(long)]
    pub all: bool,

    #[arg(long, default_value_t = 4)]
    pub concurrency: usize,

    #[arg(long, default_value_t = 5)]
    pub max_retries: u32,

    #[arg(long, default_value_t = 30)]
    pub request_timeout_seconds: u64,

    #[arg(long)]
    pub api_base_url: Option<String>,
}
