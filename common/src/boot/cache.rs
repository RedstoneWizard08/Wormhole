use rusqlite::{params, Connection};
use std::path::PathBuf;

pub struct Cache {
    conn: Connection,
}

impl Cache {
    pub fn new(path: &str) -> Result<Self, rusqlite::Error> {
        let mut dir_path = PathBuf::from(std::env::var("APPDATA").unwrap());
        dir_path.push("Wormhole");
        dir_path.push("cache");
        let conn = Connection::open(dir_path.join(path))?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cache (
                 game_id INTEGER PRIMARY KEY,
                 data TEXT
             )",
            params![],
        )?;
        Ok(Self { conn })
    }

    pub fn get(&self, game_id: i32) -> Result<Option<String>, rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM cache WHERE game_id = ?1")?;
        let row = stmt.query_row(params![game_id], |row| row.get(0));
        match row {
            Ok(data) => Ok(Some(data)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn set(&self, game_id: i32, data: &str) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "INSERT OR REPLACE INTO cache (game_id, data) VALUES (?1, ?2)",
            params![game_id, data],
        )?;
        Ok(())
    }
}

pub fn update_cache() {
    let cache = Cache::new("cache.sqlite").unwrap();
    cache.set(1, "test").unwrap();
    println!("{:?}", cache.get(1).unwrap());
}
