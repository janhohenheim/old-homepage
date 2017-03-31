use super::schema::*;


#[derive(Queryable, Identifiable)]
#[table_name="question"]
pub struct Question {
    pub id: i32,
    pub category_id: i32,
    pub text: String,
    pub answers: Vec<Answer>,
}

HasMany! {
    (question, foreign_key = category_id)
    #[table_name(category)]
    #[derive(Queryable, Identifiable)]
    pub struct Category {
        pub id: i32,
        pub text: String,
    }
}

#[derive(Queryable, Identifiable)]
#[table_name="answer"]
pub struct Answer {
    pub id: i32,
    pub text: String,
    pub is_correct: bool,
}

pub struct Player {}

pub struct Game {}
