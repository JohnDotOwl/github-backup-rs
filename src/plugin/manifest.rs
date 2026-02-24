use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub methods: Vec<String>,
    pub capabilities: Value,
}

pub fn system_manifest() -> PluginManifest {
    PluginManifest {
        name: "github-backup-rs".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        methods: vec![
            "system.info".to_string(),
            "backup.start".to_string(),
            "backup.status".to_string(),
        ],
        capabilities: serde_json::json!({
            "streaming": false,
            "long_running": false,
        }),
    }
}
