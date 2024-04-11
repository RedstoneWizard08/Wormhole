use std::path::PathBuf;

use crate::schema::instances;
use diesel::prelude::*;

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
    pub id: Option<i32>,
    pub name: String,
    pub game_id: i32,
    pub data_dir: String,
    pub cache_dir: String,
    pub install_dir: String,
    pub description: String,
    pub created: i64,
    pub updated: i64,
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
