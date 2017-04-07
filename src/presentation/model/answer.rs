#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Answer {
    pub id: i32,
    pub text: String,
    pub is_confirmed_wrong: bool,
}
