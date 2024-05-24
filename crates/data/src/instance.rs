use std::path::PathBuf;

use crate::schema::instances;
use diesel::prelude::*;

/// An instance.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Queryable,
    Identifiable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    Type,
)]
#[diesel(table_name = instances)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Instance {
    /// The instance's ID in the database.
    pub id: Option<i32>,

    /// The name of the instance.
    pub name: String,

    /// The game that this instance belongs to.
    pub game_id: i32,

    /// The data folder for this instance.
    pub data_dir: String,

    /// The cache folder for this instance.
    /// This will commonly be the same cache folder
    /// for all instances that belong to this game.
    pub cache_dir: String,

    /// Where the game was installed.
    /// This will commonly be the same folder for
    /// all instances that belong to this game.
    pub install_dir: String,

    /// The user-set description of the instance.
    /// This is blank by default.
    pub description: String,

    /// A UTC timestamp of when the instance was
    /// created.
    pub created: i64,

    /// A UTC timestamp of when the instance was
    /// last updated/changed.
    pub updated: i64,

    /// The JSON-serialized mod loader for this instance.
    pub loader: Option<String>,
}

impl Instance {
    pub fn data_dir(&self) -> PathBuf {
        PathBuf::from(self.data_dir.clone())
    }

    pub fn cache_dir(&self) -> PathBuf {
        PathBuf::from(self.cache_dir.clone())
    }

    pub fn install_dir(&self) -> PathBuf {
        PathBuf::from(self.install_dir.clone())
    }
}
