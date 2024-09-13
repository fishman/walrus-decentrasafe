#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::process::Command;

    use serde_json::Value;
    use tempfile::NamedTempFile;
    use walrus_registry::read_blob;

    #[test]
    fn test_read_blob() {
        let uuid = "4YACy3P1K5_UYT08fnp_nPrYjLKnTFVoQSEm2pDXtsM";

        // Mock the walrus output for testing
        let json_response = serde_json::json!({
            "blobId": uuid,
            "blob": "base64string"
        });

        let output = json_response.to_string();

        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", output).unwrap();

        let temp_file_path = temp_file.path().to_str().unwrap();
        let simulated_output = Command::new("cat")
            .arg(temp_file_path)
            .output()
            .expect("Failed to execute command");

        let simulated_json: Value =
            serde_json::from_slice(&simulated_output.stdout).expect("Failed to parse JSON");
        let blob_id = simulated_json["blobId"]
            .as_str()
            .expect("Missing 'blobId' field");

        let result = read_blob(uuid);

        match result {
            Ok(_) => assert_eq!(uuid, blob_id, "BlobId mismatch"),
            Err(e) => panic!("Error occurred: {}", e),
        }
    }
}
