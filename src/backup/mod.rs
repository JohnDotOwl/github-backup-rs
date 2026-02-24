pub mod account;
pub mod attachments;
pub mod gists;
pub mod hooks;
pub mod issues;
pub mod labels;
pub mod milestones;
pub mod pulls;
pub mod releases;
pub mod repositories;
pub mod security;
pub mod wiki;

use tracing::info;

use crate::{config::BackupConfig, error::Result};

#[derive(Debug, Clone)]
pub struct BackupOrchestrator {
    config: BackupConfig,
}

impl BackupOrchestrator {
    pub fn new(config: BackupConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<()> {
        info!("backup orchestration started");

        if self.config.features.repositories {
            repositories::backup_repositories(&self.config).await?;
        }
        if self.config.features.issues {
            issues::backup_issues(&self.config).await?;
        }
        if self.config.features.pulls {
            pulls::backup_pulls(&self.config).await?;
        }
        if self.config.features.releases {
            releases::backup_releases(&self.config).await?;
        }
        if self.config.features.labels {
            labels::backup_labels(&self.config).await?;
        }
        if self.config.features.milestones {
            milestones::backup_milestones(&self.config).await?;
        }
        if self.config.features.hooks {
            hooks::backup_hooks(&self.config).await?;
        }
        if self.config.features.security {
            security::backup_security(&self.config).await?;
        }
        if self.config.features.wiki {
            wiki::backup_wiki(&self.config).await?;
        }
        if self.config.features.gists {
            gists::backup_gists(&self.config).await?;
        }
        if self.config.features.account {
            account::backup_account(&self.config).await?;
        }
        if self.config.features.attachments {
            attachments::backup_attachments(&self.config).await?;
        }

        Ok(())
    }
}
