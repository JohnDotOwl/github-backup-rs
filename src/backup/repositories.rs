use std::{fs, path::Path};

use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use tracing::{info, warn};

use crate::{
    api::{client::GitHubClient, types::Repository},
    auth,
    config::{BackupConfig, BackupScope},
    error::{ApiError, BackupError, Result},
    git::subprocess,
    io::smart_write::write_json_if_changed,
};

pub async fn backup_repositories(config: &BackupConfig) -> Result<()> {
    info!("retrieving repositories");

    let token = auth::resolve_token(&config.auth)?;
    let client = GitHubClient::from_runtime(&config.runtime, token)?;
    let repositories = retrieve_repositories(config, &client).await?;

    if repositories.is_empty() {
        info!("no repositories found for this backup target");
        return Ok(());
    }

    let inventory_path = config.output_dir.join("repositories.json");
    let wrote_inventory = write_json_if_changed(&inventory_path, &repositories)?;
    if wrote_inventory {
        info!(
            path = %inventory_path.display(),
            count = repositories.len(),
            "wrote repository inventory",
        );
    } else {
        info!(
            path = %inventory_path.display(),
            count = repositories.len(),
            "repository inventory unchanged",
        );
    }

    backup_git_mirrors(config, &repositories)
}

async fn retrieve_repositories(
    config: &BackupConfig,
    client: &GitHubClient,
) -> Result<Vec<Repository>> {
    match &config.scope {
        BackupScope::User(user) => retrieve_user_repositories(client, user).await,
        BackupScope::Organization(org) => client
            .get_paginated(&format!("/orgs/{org}/repos?per_page=100&type=all"))
            .await
            .map_err(Into::into),
        BackupScope::Repositories(repositories) => {
            retrieve_selected_repositories(client, repositories).await
        }
        BackupScope::Unknown => Ok(Vec::new()),
    }
}

async fn retrieve_user_repositories(client: &GitHubClient, user: &str) -> Result<Vec<Repository>> {
    let list_path = match client.get_json::<AuthenticatedUser>("/user").await {
        Ok(authenticated) if authenticated.login.eq_ignore_ascii_case(user) => {
            "/user/repos?per_page=100&type=all&sort=full_name".to_string()
        }
        _ => format!("/users/{user}/repos?per_page=100&type=all&sort=full_name"),
    };

    client.get_paginated(&list_path).await.map_err(Into::into)
}

async fn retrieve_selected_repositories(
    client: &GitHubClient,
    repositories: &[String],
) -> Result<Vec<Repository>> {
    let mut output = Vec::with_capacity(repositories.len());

    for repository in repositories {
        let repo_path = normalize_repo_path(repository)?;
        let request_path = format!("/repos/{repo_path}");

        match client.get_json::<Repository>(&request_path).await {
            Ok(repository) => output.push(repository),
            Err(ApiError::UnexpectedStatus { status, message }) => {
                if status == StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS {
                    let legal_url = extract_legal_url(&message)
                        .unwrap_or_else(|| "https://github.com/site/policy".to_string());
                    warn!(repo = %repo_path, legal_url = %legal_url, "repository is unavailable (451), skipping");
                    continue;
                }

                if status == StatusCode::FORBIDDEN {
                    if let Some(legal_url) = extract_legal_url(&message) {
                        warn!(repo = %repo_path, legal_url = %legal_url, "repository blocked for legal reasons, skipping");
                        continue;
                    }
                }

                return Err(ApiError::UnexpectedStatus { status, message }.into());
            }
            Err(error) => return Err(error.into()),
        }
    }

    Ok(output)
}

fn normalize_repo_path(input: &str) -> Result<String> {
    if input.contains('/') {
        return Ok(input.trim_matches('/').to_string());
    }

    Err(BackupError::Config(format!(
        "repository '{input}' must be in owner/repo format"
    )))
}

fn extract_legal_url(message: &str) -> Option<String> {
    let value = serde_json::from_str::<Value>(message).ok()?;
    value
        .get("block")
        .and_then(|block| block.get("html_url"))
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn backup_git_mirrors(config: &BackupConfig, repositories: &[Repository]) -> Result<()> {
    let root = config.output_dir.join("repositories");
    fs::create_dir_all(&root)?;

    for repository in repositories {
        if let Err(error) = backup_single_repository(&root, repository) {
            warn!(
                repo = %repository.full_name,
                error = %error,
                "repository mirror step failed, continuing",
            );
        }
    }

    Ok(())
}

fn backup_single_repository(root: &Path, repository: &Repository) -> Result<()> {
    let (owner, repo_name) = repository
        .full_name
        .split_once('/')
        .unwrap_or(("unknown", repository.name.as_str()));

    let repository_root = root.join(owner).join(repo_name);
    let mirror_dir = repository_root.join("repository");
    fs::create_dir_all(&repository_root)?;

    if mirror_dir.exists() {
        info!(repo = %repository.full_name, path = %mirror_dir.display(), "updating repository mirror");
        subprocess::fetch_mirror(&mirror_dir)?;
    } else {
        info!(repo = %repository.full_name, path = %mirror_dir.display(), "cloning repository mirror");
        subprocess::clone_mirror(&repository.clone_url, &mirror_dir)?;
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct AuthenticatedUser {
    login: String,
}
