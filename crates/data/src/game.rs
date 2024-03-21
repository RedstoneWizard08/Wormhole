use crate::schema::games;
use diesel::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Queryable, Identifiable, Selectable)]
#[diesel(table_name = games)]
pub struct Game {
    pub id: Option<i32>,
    pub name: String,
}
