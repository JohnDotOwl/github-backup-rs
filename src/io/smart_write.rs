use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer, Value};

use super::atomic_write::write_atomic;

pub fn write_if_changed(path: &Path, bytes: &[u8]) -> std::io::Result<bool> {
    if path.exists() {
        let current = fs::read(path)?;
        if current == bytes {
            return Ok(false);
        }
    }

    write_atomic(path, bytes)?;
    Ok(true)
}

pub fn write_json_if_changed<T: Serialize>(path: &Path, value: &T) -> std::io::Result<bool> {
    let mut json_value = serde_json::to_value(value).map_err(as_io_error)?;
    sort_json(&mut json_value);

    let bytes = to_pretty_json_bytes(&json_value).map_err(as_io_error)?;
    write_if_changed(path, &bytes)
}

fn sort_json(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for nested in map.values_mut() {
                sort_json(nested);
            }
            let sorted = map
                .iter()
                .map(|(key, val)| (key.clone(), val.clone()))
                .collect::<BTreeMap<_, _>>();
            map.clear();
            for (key, val) in sorted {
                map.insert(key, val);
            }
        }
        Value::Array(items) => {
            for item in items {
                sort_json(item);
            }
        }
        _ => {}
    }
}

fn to_pretty_json_bytes(value: &Value) -> serde_json::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut serializer = Serializer::with_formatter(&mut buffer, formatter);
    value.serialize(&mut serializer)?;
    buffer.write_all(b"\n").map_err(as_serde_error)?;
    Ok(buffer)
}

fn as_io_error(error: impl std::error::Error) -> std::io::Error {
    std::io::Error::other(error.to_string())
}

fn as_serde_error(error: std::io::Error) -> serde_json::Error {
    serde_json::Error::io(error)
}

#[allow(dead_code)]
fn _to_pathbuf(path: &Path) -> PathBuf {
    path.to_path_buf()
}
