CREATE TABLE blobs (
    uuid TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    sha256digest TEXT,
    data BLOB NOT NULL
);
CREATE INDEX idx_blobs_name ON blobs (name);
CREATE INDEX idx_blobs_name_sha256digest ON blobs (name, sha256digest);
