use crate::error::GitError;

pub fn https_url(
    owner: &str,
    repo: &str,
    token: Option<&str>,
) -> std::result::Result<String, GitError> {
    if owner.is_empty() || repo.is_empty() {
        return Err(GitError::InvalidUrl(
            "owner and repository name must be non-empty".to_string(),
        ));
    }

    let host = "github.com";
    let path = format!("{owner}/{repo}.git");
    let url = match token {
        Some(token) => format!("https://{token}:x-oauth-basic@{host}/{path}"),
        None => format!("https://{host}/{path}"),
    };

    Ok(url)
}

pub fn ssh_url(owner: &str, repo: &str) -> std::result::Result<String, GitError> {
    if owner.is_empty() || repo.is_empty() {
        return Err(GitError::InvalidUrl(
            "owner and repository name must be non-empty".to_string(),
        ));
    }

    Ok(format!("git@github.com:{owner}/{repo}.git"))
}
