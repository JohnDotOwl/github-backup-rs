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
    pub features: BackupFeatures,
    pub runtime: RuntimeConfig,
    pub plugin_mode: bool,
}

impl BackupConfig {
    pub fn from_cli(args: &CliArgs) -> Result<Self> {
        let mut features = BackupFeatures {
            repositories: args.include_repositories,
            issues: args.include_issues,
            pulls: args.include_pulls,
            releases: args.include_releases,
            labels: args.include_labels,
            milestones: args.include_milestones,
            hooks: args.include_hooks,
            security: args.include_security,
            wiki: args.include_wiki,
            gists: args.include_gists,
            account: args.include_account,
            attachments: args.include_attachments,
        };

        if args.all {
            features.enable_all();
        }

        if !features.any_enabled() {
            features.repositories = true;
        }

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
            features,
            runtime: RuntimeConfig {
                concurrency: args.concurrency,
                max_retries: args.max_retries,
                request_timeout_seconds: args.request_timeout_seconds,
                api_base_url: args
                    .api_base_url
                    .clone()
                    .unwrap_or_else(|| "https://api.github.com".to_string()),
            },
            plugin_mode: args.plugin,
        };

        config.validate()?;
        Ok(config)
    }

    pub fn from_rpc_params(params: serde_json::Value) -> Result<Self> {
        let config: Self = serde_json::from_value(params)
            .map_err(|error| BackupError::Config(format!("invalid plugin params: {error}")))?;
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

        if !self.plugin_mode && matches!(self.scope, BackupScope::Unknown) {
            return Err(BackupError::Config(
                "target argument is required outside plugin mode".to_string(),
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupFeatures {
    pub repositories: bool,
    pub issues: bool,
    pub pulls: bool,
    pub releases: bool,
    pub labels: bool,
    pub milestones: bool,
    pub hooks: bool,
    pub security: bool,
    pub wiki: bool,
    pub gists: bool,
    pub account: bool,
    pub attachments: bool,
}

impl BackupFeatures {
    pub fn enable_all(&mut self) {
        self.repositories = true;
        self.issues = true;
        self.pulls = true;
        self.releases = true;
        self.labels = true;
        self.milestones = true;
        self.hooks = true;
        self.security = true;
        self.wiki = true;
        self.gists = true;
        self.account = true;
        self.attachments = true;
    }

    pub fn any_enabled(&self) -> bool {
        self.repositories
            || self.issues
            || self.pulls
            || self.releases
            || self.labels
            || self.milestones
            || self.hooks
            || self.security
            || self.wiki
            || self.gists
            || self.account
            || self.attachments
    }
}
