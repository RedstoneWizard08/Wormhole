use crate::schema::games;
use diesel::prelude::*;

/// A supported game. This only holds metadata.
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
#[diesel(table_name = games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    /// The game's ID in the database.
    pub id: Option<i32>,

    /// The human-friendly name of the game.
    pub name: String,
}
