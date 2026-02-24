use crate::error::Result;

use super::AuthProvider;

pub struct GitHubAppAuth {
    pub jwt: String,
}

impl AuthProvider for GitHubAppAuth {
    fn auth_header_value(&self) -> Result<String> {
        Ok(format!("Bearer {}", self.jwt))
    }
}
