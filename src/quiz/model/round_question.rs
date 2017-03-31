extern crate chrono;

use super::super::schema::round_question;
use self::chrono::NaiveDateTime;
use super::question::Question;
use super::round::Round;


#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Question)]
#[belongs_to(Round)]
#[table_name="round_question"]
pub struct RoundQuestion {
    pub id: i32,
    pub round_id: i32,
    pub question_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub is_joker_used: bool,
    pub answer_id: Option<i32>,
}
