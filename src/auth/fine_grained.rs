use crate::error::Result;

use super::AuthProvider;

pub struct FineGrainedPatAuth {
    pub token: String,
}

impl AuthProvider for FineGrainedPatAuth {
    fn auth_header_value(&self) -> Result<String> {
        Ok(format!("token {}", self.token))
    }
}
