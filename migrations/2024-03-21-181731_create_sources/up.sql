CREATE TABLE IF NOT EXISTS sources (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    base_url TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS supported_sources (
    id INTEGER PRIMARY KEY,
    is_supported BOOLEAN NOT NULL DEFAULT TRUE,
    source_id INTEGER NOT NULL REFERENCES sources(id),
    game_id INTEGER NOT NULL REFERENCES games(id)
);
