use data::schema::{round, round_question, round_category};
use super::player::Player;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Player)]
#[has_many(round_question)]
#[has_many(round_category)]
#[table_name="round"]
pub struct Round {
    pub id: i32,
    pub player_id: i32,
    pub is_finished: bool,
}

#[derive(Insertable)]
#[table_name="round"]
pub struct NewRound {
    pub player_id: i32,
}
