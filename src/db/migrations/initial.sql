CREATE TABLE files (
    id int PRIMARY KEY ASC,
    path varchar(2000),
    name varchar(2000),
    hash_sum text
);

CREATE TABLE tags (
    id int PRIMARY KEY ASC,
    name varchar(2000) UNIQUE NOT NULL
);

CREATE TABLE file_tags (
    file_id int REFERENCES files (id),
    tag_id int REFERENCES tags (id)
);
