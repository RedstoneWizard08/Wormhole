CREATE TABLE IF NOT EXISTS mods (
    id INTEGER PRIMARY KEY,
    mod_id TEXT NOT NULL,
    version_id TEXT,
    name TEXT NOT NULL,
    file_name TEXT NOT NULL,
    source_id INTEGER REFERENCES sources(id),
    instance_id INTEGER REFERENCES instances(id),
    size INT,
    hash TEXT
);
