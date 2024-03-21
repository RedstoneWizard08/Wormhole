CREATE TABLE IF NOT EXISTS instance_meta (
    id INTEGER PRIMARY KEY,
    instance_id INTEGER NOT NULL REFERENCES instances(id),
    game_id INTEGER NOT NULL REFERENCES games(id),
    data_dir TEXT NOT NULL,
    cache_dir TEXT NOT NULL,
    description TEXT NOT NULL,
    created BIG INT NOT NULL,
    updated BIG INT NOT NULL
);
