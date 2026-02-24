use serde_json::{json, Value};

use crate::{backup::BackupOrchestrator, config::BackupConfig};

use super::manifest;

pub async fn dispatch_line(line: &str) -> Option<String> {
    if line.trim().is_empty() {
        return None;
    }

    let request: Value = match serde_json::from_str(line) {
        Ok(request) => request,
        Err(error) => {
            return Some(error_response(
                None,
                -32700,
                format!("parse error: {error}"),
            ))
        }
    };

    let id = request.get("id").cloned();
    let method = request
        .get("method")
        .and_then(Value::as_str)
        .unwrap_or_default();

    match method {
        "system.info" => Some(success_response(id, json!(manifest::system_manifest()))),
        "backup.start" => {
            let params = request.get("params").cloned().unwrap_or_else(|| json!({}));
            let config = match BackupConfig::from_rpc_params(params) {
                Ok(config) => config,
                Err(error) => return Some(error_response(id, -32602, error.to_string())),
            };

            let result = BackupOrchestrator::new(config).run().await;
            match result {
                Ok(()) => Some(success_response(id, json!({ "status": "completed" }))),
                Err(error) => Some(error_response(id, -32001, error.to_string())),
            }
        }
        "backup.status" => Some(success_response(
            id,
            json!({
                "state": "idle",
                "message": "no active background jobs in this scaffold"
            }),
        )),
        _ => Some(error_response(
            id,
            -32601,
            format!("unknown method: {method}"),
        )),
    }
}

fn success_response(id: Option<Value>, result: Value) -> String {
    json!({
        "jsonrpc": "2.0",
        "id": id.unwrap_or(Value::Null),
        "result": result,
    })
    .to_string()
}

fn error_response(id: Option<Value>, code: i64, message: String) -> String {
    json!({
        "jsonrpc": "2.0",
        "id": id.unwrap_or(Value::Null),
        "error": {
            "code": code,
            "message": message,
        },
    })
    .to_string()
}
