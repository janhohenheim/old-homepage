#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Answer {
    id: i32,
    text: String,
    is_confirmed_wrong: bool,
}
