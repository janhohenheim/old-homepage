extern crate chrono;

use std::result;
use self::chrono::NaiveDateTime;
use super::quiz_error::QuizError;
use data::model::quiz::category::Category;
use data::model::quiz::player::Player;
use business::crud::*;

type Result<T> = result::Result<T, QuizError>;

pub struct RoundData {
    pub id: i32,
    pub player: Player,
    pub categories: Vec<Category>,
    pub answer_count: usize,
    pub is_last_answer_wrong: bool,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

pub fn get_round_data(round_id: i32) -> Result<RoundData> {
    let round = get_round(round_id)?;
    if !round.is_finished {
        return Err(QuizError::GameStillInProgress);
    }
    let round_questions = get_round_questions(round_id)?;
    if round_questions.is_empty() {
        return Err(QuizError::StateError);
    }
    let categories = get_round_categories_joined(round_id)?;

    let first_round_question = &round_questions[0];
    let last_round_question = &round_questions[round_questions.len() - 1];

    let end_time = last_round_question.end_time
        .ok_or_else(|| QuizError::StateError)?;

    let is_last_answer_wrong = match last_round_question.answer_id {
        Some(answer_id) => !get_answer(answer_id)?.is_correct,
        None => false,
    };

    let answer_count = round_questions
        .iter()
        .filter(|x| x.answer_id.is_some())
        .count();

    let player = get_player(round.player_id)?;

    Ok(RoundData {
           id: round.id,
           player: player,
           categories: categories,
           answer_count: answer_count,
           is_last_answer_wrong: is_last_answer_wrong,
           start_time: first_round_question.start_time,
           end_time: end_time,
       })
}

pub fn get_all_round_data() -> Result<Vec<RoundData>> {
    let rounds = get_rounds()?;
    let mut round_data = Vec::new();
    for round in rounds {
        round_data.push(get_round_data(round.id)?);
    }
    Ok(round_data)
}
