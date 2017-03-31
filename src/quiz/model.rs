use super::schema::*;

#[table_name="category"]
#[derive(Queryable, Identifiable, Associations, Debug)]
#[has_many(question)]
pub struct Category {
        pub id: i32,
        pub text: String,
}


#[table_name="question"]
#[derive(Queryable, Identifiable, Associations)]
#[has_many(answer)]
#[belongs_to(Category)]
pub struct Question {
    pub id: i32,
    pub category_id: i32,
    pub text: String
}

#[table_name="answer"]
#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Question)]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub text: String,
    pub is_correct: bool,
}


pub struct Player {}

pub struct Game {}
