use super::super::schema::{round, round_question};
use super::player::Player;
use super::category::Category;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Category)]
#[belongs_to(Player)]
#[has_many(round_question)]
#[table_name="round"]
pub struct Round {
    pub id: i32,
    pub category_id: i32,
    pub player_id: i32,
}

#[derive(Insertable)]
#[table_name="round"]
pub struct NewRound {
    pub category_id: i32,
    pub player_id: i32,
}
