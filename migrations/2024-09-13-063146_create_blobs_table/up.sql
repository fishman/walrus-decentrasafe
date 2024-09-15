CREATE TABLE blobs (
    uuid TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    digest TEXT,
    content_length INT,
    data BLOB NOT NULL,
    UNIQUE (name, digest)
);
CREATE INDEX idx_blobs_name ON blobs (name);
CREATE INDEX idx_blobs_name_digest ON blobs (name, digest);
