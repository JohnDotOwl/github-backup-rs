use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use crate::error::Result;

use super::AuthProvider;

pub struct ClassicPatAuth {
    pub username: String,
    pub token: String,
}

impl AuthProvider for ClassicPatAuth {
    fn auth_header_value(&self) -> Result<String> {
        let encoded = STANDARD.encode(format!("{}:{}", self.username, self.token));
        Ok(format!("Basic {encoded}"))
    }
}
