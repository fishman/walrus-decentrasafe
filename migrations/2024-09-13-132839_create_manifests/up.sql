CREATE TABLE manifests (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    reference TEXT NOT NULL,
    content BLOB NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    UNIQUE(name, reference)               -- Ensure uniqueness of name and reference
);
