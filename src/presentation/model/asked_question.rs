use super::answer::Answer;
use std::vec::Vec;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct AskedQuestion {
    pub text: String,
    pub answers: Vec<Answer>,
}
