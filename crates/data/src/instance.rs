use std::path::PathBuf;

use crate::schema::{instance_meta, instances};
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
}

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
    Associations,
    Insertable,
    Serialize,
    Deserialize,
    Type,
)]
#[diesel(table_name = instance_meta)]
#[diesel(belongs_to(Instance))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InstanceMeta {
    pub id: Option<i32>,
    pub instance_id: i32,
    pub game_id: i32,
    pub data_dir: String,
    pub cache_dir: String,
    pub description: String,
    pub created: i64,
    pub updated: i64,
}

impl InstanceMeta {
    pub fn data_dir(&self) -> PathBuf {
        PathBuf::from(self.data_dir.clone())
    }

    pub fn cache_dir(&self) -> PathBuf {
        PathBuf::from(self.cache_dir.clone())
    }
}
