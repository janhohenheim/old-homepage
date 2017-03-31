use super::schema::*;

#[derive(Queryable, Identifiable, Associations, Debug)]
#[has_many(question)]
#[table_name="category"]
pub struct Category {
    pub id: i32,
    pub text: String,
    pub is_active: bool,
}

#[derive(Insertable)]
#[table_name="category"]
pub struct NewCategory<'a> {
    pub text: &'a str,
}


#[derive(Queryable, Identifiable, Associations)]
#[has_many(answer)]
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

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Question)]
#[table_name="answer"]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub text: String,
    pub is_correct: bool,
    pub is_active: bool,
}

#[derive(Insertable)]
#[table_name="answer"]
pub struct NewAnswer<'a> {
    pub question_id: i32,
    pub text: &'a str,
    pub is_correct: bool,
}


pub struct Player {}

pub struct Game {}
