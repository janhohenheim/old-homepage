use data::schema::{question, answer, round_question};
use super::category::Category;

#[derive(Queryable, Identifiable, Associations)]
#[has_many(answer)]
#[has_many(round_question)]
#[belongs_to(Category)]
#[table_name="question"]
pub struct Question {
    pub id: i32,
    pub category_id: i32,
    pub text: String,
    pub is_active: bool,
}

#[derive(Insertable)]
#[table_name="question"]
pub struct NewQuestion<'a> {
    pub category_id: i32,
    pub text: &'a str,
}
