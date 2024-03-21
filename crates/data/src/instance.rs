use crate::schema::{instance_meta, instances};
use diesel::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Queryable, Identifiable, Selectable)]
#[diesel(table_name = instances)]
pub struct Instance {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Queryable, Identifiable, Selectable, Associations,
)]
#[diesel(table_name = instance_meta)]
#[diesel(belongs_to(Instance))]
pub struct InstanceMeta {
    pub id: Option<i32>,
    pub instance_id: i32,
    pub game_id: i32,
    pub data_dir: String,
    pub cache_dir: String,
    pub description: String,
    pub created: u64,
    pub updated: u64,
}
