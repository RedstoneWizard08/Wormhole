use crate::schema::mods;
use diesel::prelude::*;

/// An installed mod.
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
    /// The mod's ID in the database.
    pub id: Option<i32>,

    /// The mod's ID in its source.
    pub mod_id: String,

    /// The ID of the installed version of the mod.
    pub version_id: Option<String>,

    /// The mod's name (human-friendly).
    pub name: String,

    /// The name of the installed file.
    pub file_name: String,

    /// The instance this mod was installed to.
    pub instance_id: Option<i32>,

    /// The source that this mod belongs to.
    pub source_id: Option<i32>,

    /// The size of the installed file.
    pub size: Option<i32>,

    /// The SHA-512 hash of the installed file.
    pub hash: Option<String>,

    /// A JSON-serialized list of paths, relative
    /// to the instance's root data directory, that
    /// were created when this mod was installed.
    pub path: String,
}
