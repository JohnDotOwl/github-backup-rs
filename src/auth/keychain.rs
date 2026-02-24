use crate::{error::AuthError, Result};

#[cfg(target_os = "macos")]
pub fn read_token_from_keychain(service: Option<&str>) -> Result<Option<String>> {
    let _service = service.unwrap_or("github-backup-rs");
    Err(AuthError::Keychain(
        "keychain lookup is planned but not implemented in this scaffold".to_string(),
    )
    .into())
}

#[cfg(not(target_os = "macos"))]
pub fn read_token_from_keychain(_service: Option<&str>) -> Result<Option<String>> {
    Ok(None)
}
