#[derive(Queryable, Identifiable)]
#[table_name="category"]
pub struct Category {
    pub id: i32,
    pub text: String,
    pub questions: Vec<Question>,
}

#[derive(Queryable, Identifiable)]
#[table_name="question"]
pub struct Question {
    pub id: i32,
    pub text: String,
    pub answers: Vec<Answer>,
}

HasMany! {
    (questions, foreign_key = category_iq)
    #[derive(Queryable, Identifiable)]
    #[table_name="category"]
    pub struct Category {
        pub id: i32,
        pub text: String,
        pub questions: Vec<Question>,
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
