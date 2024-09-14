use base64::{engine::general_purpose::STANDARD, Engine};

use serde_json::Value;
use std::process::Command;

pub fn store_blob(filename: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let output = Command::new("walrus")
        .arg("--json")
        .arg("store")
        .arg(filename)
        .output()?;

    if !output.status.success() {
        return Err(format!("Command failed with status: {:?}", output.status).into());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&output_str)?;

    let blob_id = json["alreadyCertified"]["blobId"]
        .as_str()
        .ok_or("Missing blobId")?
        .to_string();

    let tx_digest = json["alreadyCertified"]["event"]["txDigest"]
        .as_str()
        .ok_or("Missing txDigest")?
        .to_string();

    Ok((blob_id, tx_digest))
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

