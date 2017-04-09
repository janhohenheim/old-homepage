#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Rank {
    pub round_id: i32,
    pub name: String,
    pub score: i32,
    pub points: i32,
    pub game_start: String,
    pub game_length: i32,
    pub categories: String,
}
