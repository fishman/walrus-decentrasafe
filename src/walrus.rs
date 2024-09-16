use base64::{engine::general_purpose::STANDARD, Engine};

use serde_json::Value;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn store_blob(data: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(&data)?;
    temp_file.flush()?;

    let output = Command::new("walrus")
        .arg("--json")
        .arg("store")
        .arg(temp_file.path())
        .output()?;

    if !output.status.success() {
        //log::info!("{}", temp_file);
        log::info!("{}", String::from_utf8(data)?);
        return Err(format!("Command failed with status: {:?}", output.status).into());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&output_str)?;
    log::info!("{}", json);
    let mut blob_id: String = "".to_string();

    if json["newlyCreated"].is_object() {
        blob_id = json["newlyCreated"]["blobObject"]["blobId"]
            .as_str()
            .ok_or("Missing blobId")?
            .to_string();
    } else if json["alreadyCertified"].is_object() {
        blob_id = json["alreadyCertified"]["blobId"]
            .as_str()
            .ok_or("Missing blobId")?
            .to_string();

        //let tx_digest = json["alreadyCertified"]["event"]["txDigest"]
        //    .as_str()
        //    .ok_or("Missing txDigest")?
        //    .to_string();
    }

    //Ok((blob_id, tx_digest))
    Ok(blob_id)
}

pub fn read_blob(uuid: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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

    Ok(blob)
}
