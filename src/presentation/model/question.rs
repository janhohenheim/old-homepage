use super::category::Category;
use super::answer::Answer;
use std::vec::Vec;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Question {
    pub id: i32,
    pub text: String,
    pub category: Category,
    pub answers: Vec<Answer>,
}
