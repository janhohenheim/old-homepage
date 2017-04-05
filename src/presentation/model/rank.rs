#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Rank {
    ranking: i32,
    name: String,
    score: i32,
    points: i32,
    game_start: String,
    game_length: i32,
    categories: String,
}