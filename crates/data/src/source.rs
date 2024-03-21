use crate::{
    game::Game,
    schema::{sources, supported_sources},
};
use diesel::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Queryable, Identifiable, Selectable)]
#[diesel(table_name = sources)]
pub struct Source {
    pub id: Option<i32>,
    pub name: String,
    pub base_url: String,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Queryable, Identifiable, Selectable, Associations,
)]
#[diesel(table_name = supported_sources)]
#[diesel(belongs_to(Game))]
#[diesel(belongs_to(Source))]
#[diesel(primary_key(source_id, game_id))]
pub struct SupportedSource {
    pub id: Option<i32>,
    pub is_supported: bool,
    pub source_id: i32,
    pub game_id: i32,
}
