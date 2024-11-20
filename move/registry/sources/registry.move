/// Module: registry
module registry::storage {

    use std::signer;
    use std::vector;
    use std::option::{self, Option};
    use std::string::{self, String};
    use std::timestamp;

    struct Blob has store {
        uuid: String,
        name: String,
        digest: Option<String>,
        content_length: Option<u64>,
        data: vector<u8>,
        walrus_blob_id: Option<String>,
    }

    struct Manifest has store {
        id: u64,
        name: String,
        reference: String,
        content: vector<u8>,
        created_at: u64,
        updated_at: u64,
    }

    struct BlobRegistry has store {
        blobs: vector<Blob>,
    }

    struct ManifestRegistry has store {
        manifests: vector<Manifest>,
    }

    public fun initialize_registry(account: &signer) {
        move_to(account, BlobRegistry { blobs: vector::empty() });
        move_to(account, ManifestRegistry { manifests: vector::empty() });
    }

    public fun start_blob_upload(
        registry: &mut BlobRegistry,
        name: String
    ): String {
        let uuid = generate_uuid();
        let new_blob = Blob {
            uuid: uuid.clone(),
            name,
            digest: option::none(),
            content_length: option::none(),
            data: vector::empty(),
            walrus_blob_id: option::none(),
        };
        vector::push_back(&mut registry.blobs, new_blob);
        uuid
    }

    public fun complete_blob_upload(
        registry: &mut BlobRegistry,
        uuid: String,
        name: String,
        data: vector<u8>
    ): bool {
        let blob_index = find_blob_by_uuid_and_name(&registry.blobs, &uuid, &name);
        if (blob_index == option::none()) {
            return false;
        }

        let index = option::extract(blob_index);
        let digest = calculate_sha256_digest(&data);
        let content_length = vector::length(&data) as u64;

        let blob = &mut registry.blobs[index];
        blob.digest = option::some(digest);
        blob.content_length = option::some(content_length);
        blob.data = data;
        true
    }

    public fun check_blob(
        registry: &BlobRegistry,
        name: String,
        digest: String
    ): Option<u64> {
        for blob in &registry.blobs {
            if (blob.name == name && option::is_some(&blob.digest) && option::extract(&blob.digest) == digest) {
                return option::some(blob.content_length.unwrap_or(0));
            }
        }
        option::none()
    }

    public fun upload_manifest(
        registry: &mut ManifestRegistry,
        name: String,
        reference: String,
        content: vector<u8>
    ) {
        let timestamp = timestamp::now_seconds();
        let new_manifest = Manifest {
            id: vector::length(&registry.manifests) as u64,
            name,
            reference,
            content,
            created_at: timestamp,
            updated_at: timestamp,
        };
        vector::push_back(&mut registry.manifests, new_manifest);
    }

    public fun check_manifest(
        registry: &ManifestRegistry,
        name: String,
        reference: String
    ): Option<u64> {
        for manifest in &registry.manifests {
            if (manifest.name == name && manifest.reference == reference) {
                return option::some(manifest.id);
            }
        }
        option::none()
    }

    public fun fetch_manifest(
        registry: &ManifestRegistry,
        name: String,
        reference: String
    ): Option<Manifest> {
        for manifest in &registry.manifests {
            if (manifest.name == name && manifest.reference == reference) {
                return option::some(copy manifest);
            }
        }
        option::none()
    }

    // Helper functions
    fun find_blob_by_uuid_and_name(blobs: &vector<Blob>, uuid: &String, name: &String): Option<u64> {
        let i = 0;
        for blob in blobs {
            if (blob.uuid == *uuid && blob.name == *name) {
                return option::some(i);
            }
            i = i + 1;
        }
        option::none()
    }

    fun generate_uuid(): String {
        // Simplified placeholder, replace with a real UUID generator
        string::from_utf8(vector::from_bytes("uuid-placeholder"))
    }

    fun calculate_sha256_digest(data: &vector<u8>): String {
        // Simplified placeholder for digest calculation
        string::from_utf8(vector::from_bytes("sha256-placeholder"))
    }
}
