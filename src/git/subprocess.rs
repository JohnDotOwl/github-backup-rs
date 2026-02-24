use std::{path::Path, process::Command};

use crate::error::GitError;

pub fn clone_mirror(url: &str, destination: &Path) -> std::result::Result<(), GitError> {
    run_git_command(
        &[
            "clone",
            "--mirror",
            url,
            destination.to_string_lossy().as_ref(),
        ],
        None,
    )
}

pub fn fetch_mirror(destination: &Path) -> std::result::Result<(), GitError> {
    run_git_command(&["fetch", "--all", "--prune"], Some(destination))
}

pub fn ls_remote(url: &str) -> std::result::Result<(), GitError> {
    run_git_command(&["ls-remote", "--heads", url], None)
}

fn run_git_command(args: &[&str], workdir: Option<&Path>) -> std::result::Result<(), GitError> {
    let mut command = Command::new("git");
    command.args(args);
    if let Some(workdir) = workdir {
        command.current_dir(workdir);
    }

    let output = command.output().map_err(|source| GitError::Io { source })?;
    if output.status.success() {
        return Ok(());
    }

    Err(GitError::CommandFailed {
        command: format!("git {}", args.join(" ")),
        status: output.status.code(),
        stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
    })
}
