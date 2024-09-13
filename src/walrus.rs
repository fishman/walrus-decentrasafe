use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use serde_json::Value;
use std::process::Command;

pub fn store_blob(binary_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("walrus")
        .arg("--json")
        .arg("store")
        .arg(binary_path)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Error storing blob: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}

pub fn read_blob(uuid: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("walrus")
        .arg("--json")
        .arg("read")
        .arg(uuid)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Error reading blob: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let json_output: Value = serde_json::from_slice(&output.stdout)?;
    let base64_blob = json_output["blob"].as_str().ok_or("Missing 'blob' field")?;
    let blob = STANDARD.decode(base64_blob)?;

    Ok(String::from_utf8(blob)?)
}

