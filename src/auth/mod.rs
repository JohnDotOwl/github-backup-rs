pub mod classic_pat;
pub mod file_token;
pub mod fine_grained;
pub mod github_app;
pub mod keychain;

use crate::{config::AuthConfig, error::Result};

pub trait AuthProvider {
    fn auth_header_value(&self) -> Result<String>;
}

pub fn resolve_token(config: &AuthConfig) -> Result<Option<String>> {
    if let Some(token) = &config.token {
        return Ok(Some(token.clone()));
    }

    if let Some(path) = &config.token_file {
        return file_token::read_token_file(path).map(Some);
    }

    if config.use_keychain {
        return keychain::read_token_from_keychain(config.keychain_service.as_deref());
    }

    Ok(None)
}
