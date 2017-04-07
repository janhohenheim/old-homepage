use super::category::Category;
use super::answer::Answer;
use std::vec::Vec;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Question<'a> {
    pub id: i32,
    pub text: String,
    // hack because of limitations of handlebars-rs
    pub categories: &'a Vec<Category>,
    pub category: Category,
    pub answers: Vec<Answer>,
}
