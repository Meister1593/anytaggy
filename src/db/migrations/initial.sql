CREATE TABLE files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path varchar(2000) NOT NULL,
    name varchar(2000) NOT NULL,
    hash_sum text UNIQUE NOT NULL
);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name varchar(2000) UNIQUE NOT NULL
);

CREATE TABLE file_tags (
    file_id INTEGER REFERENCES files (id),
    tag_id INTEGER REFERENCES tags (id)
);
