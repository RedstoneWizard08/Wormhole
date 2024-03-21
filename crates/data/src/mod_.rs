use crate::schema::mods;
use diesel::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Queryable, Identifiable, Selectable)]
#[diesel(table_name = mods)]
pub struct Mod {
    pub id: Option<i32>,
    pub mod_id: String,
    pub version_id: Option<String>,
    pub name: String,
    pub file_name: String,
    pub source_id: Option<i32>,
    pub size: usize,
    pub hash: String,
}
