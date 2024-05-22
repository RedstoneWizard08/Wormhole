use crate::schema::mods;
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
#[diesel(table_name = mods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DbMod {
    pub id: Option<i32>,
    pub mod_id: String,
    pub version_id: Option<String>,
    pub name: String,
    pub file_name: String,
    pub instance_id: Option<i32>,
    pub source_id: Option<i32>,
    pub size: Option<i32>,
    pub hash: Option<String>,
}
