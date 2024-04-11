CREATE TABLE IF NOT EXISTS instances (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    game_id INTEGER NOT NULL REFERENCES games(id),
    data_dir TEXT NOT NULL,
    cache_dir TEXT NOT NULL,
    description TEXT NOT NULL,
    created BIG INT NOT NULL,
    updated BIG INT NOT NULL,
    install_dir TEXT NOT NULL
);
