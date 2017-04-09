extern crate chrono;

use std::result;
use self::chrono::NaiveDateTime;
use super::quiz_error::QuizError;
use data::model::quiz::category::Category;
use business::crud::*;

type Result<T> = result::Result<T, QuizError>;

pub struct RoundData {
    pub id: i32,
    pub categories: Vec<Category>,
    pub answer_count: i32,
    pub is_last_answer_correct: bool,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

pub fn get_round_data(round_id: i32) -> Result<RoundData> {
    Err(QuizError::StateError)
}

pub fn get_all_round_data() -> Result<Vec<RoundData>> {
    Err(QuizError::StateError)
}
