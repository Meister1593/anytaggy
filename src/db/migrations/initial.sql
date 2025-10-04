CREATE TABLE files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL,
    name TEXT NOT NULL,
    contents_hash TEXT UNIQUE NOT NULL,
    fingerprint_hash TEXT UNIQUE NOT NULL
);
CREATE INDEX idx_files_path ON files (path);
CREATE INDEX idx_files_name ON files (name);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);
CREATE INDEX idx_tags_name ON tags (name);

CREATE TABLE file_tags (
    file_id INTEGER REFERENCES files (id) ON DELETE CASCADE,
    tag_id INTEGER REFERENCES tags (id) ON DELETE CASCADE,
    UNIQUE (file_id, tag_id)
);
CREATE INDEX idx_file_tags_file_id ON file_tags (file_id);
CREATE INDEX idx_file_tags_tag_id ON file_tags (tag_id);
