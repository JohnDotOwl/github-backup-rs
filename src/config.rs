use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    cli::args::CliArgs,
    error::{BackupError, Result},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub scope: BackupScope,
    pub output_dir: PathBuf,
    pub auth: AuthConfig,
    pub runtime: RuntimeConfig,
}

impl BackupConfig {
    pub fn from_cli(args: &CliArgs) -> Result<Self> {
        let scope = if !args.repositories.is_empty() {
            BackupScope::Repositories(args.repositories.clone())
        } else if let Some(target) = &args.target {
            if args.organization {
                BackupScope::Organization(target.clone())
            } else {
                BackupScope::User(target.clone())
            }
        } else {
            BackupScope::Unknown
        };

        let config = Self {
            scope,
            output_dir: args.output_dir.clone(),
            auth: AuthConfig {
                token: args.token.clone(),
                token_file: args.token_file.clone(),
                use_keychain: args.use_keychain,
                keychain_service: args.keychain_service.clone(),
            },
            runtime: RuntimeConfig {
                concurrency: args.concurrency,
                max_retries: args.max_retries,
                request_timeout_seconds: args.request_timeout_seconds,
                api_base_url: args
                    .api_base_url
                    .clone()
                    .unwrap_or_else(|| "https://api.github.com".to_string()),
            },
        };

        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        if self.runtime.concurrency == 0 {
            return Err(BackupError::Config(
                "concurrency must be greater than 0".to_string(),
            ));
        }

        if self.runtime.max_retries > 20 {
            return Err(BackupError::Config(
                "max_retries must be less than or equal to 20".to_string(),
            ));
        }

        if matches!(self.scope, BackupScope::Unknown) {
            return Err(BackupError::Config(
                "target argument is required (or use --repo owner/repo)".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BackupScope {
    User(String),
    Organization(String),
    Repositories(Vec<String>),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub token: Option<String>,
    pub token_file: Option<PathBuf>,
    pub use_keychain: bool,
    pub keychain_service: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub concurrency: usize,
    pub max_retries: u32,
    pub request_timeout_seconds: u64,
    pub api_base_url: String,
}
