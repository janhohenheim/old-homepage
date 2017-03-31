use super::super::schema::{round, player};

#[derive(Queryable, Identifiable, Associations)]
#[has_many(round)]
#[table_name="player"]
pub struct Player {
    pub id: i32,
    pub name: String,
}
