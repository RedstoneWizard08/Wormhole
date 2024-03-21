CREATE TABLE IF NOT EXISTS mods (
    id INTEGER PRIMARY KEY,
    mod_id TEXT NOT NULL,
    version_id TEXT,
    name TEXT NOT NULL,
    file_name TEXT NOT NULL,
    source_id INTEGER REFERENCES sources(id),
    size INT NOT NULL,
    hash TEXT NOT NULL
);
