use std::{fs, path::Path};

use crate::error::{AuthError, Result};

pub fn read_token_file(path: &Path) -> Result<String> {
    let token = fs::read_to_string(path).map_err(|source| AuthError::TokenFileRead {
        path: path.display().to_string(),
        source,
    })?;

    let trimmed = token.trim().to_string();
    if trimmed.is_empty() {
        return Err(
            AuthError::InvalidConfig(format!("token file '{}' is empty", path.display())).into(),
        );
    }

    Ok(trimmed)
}
