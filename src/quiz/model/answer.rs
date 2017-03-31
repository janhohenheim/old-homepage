use super::super::schema::answer;
use super::question::Question;

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
