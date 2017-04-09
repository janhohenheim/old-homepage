extern crate diesel;
extern crate rand;

use self::diesel::result::Error as DatabaseError;
use self::rand::{thread_rng, Rng};

use std::vec::Vec;
use std::error;
use std::fmt;
use std::result;
use std::convert::From;

use super::crud::*;

use data::model::quiz::category::Category;
use data::model::quiz::question::Question;
use data::model::quiz::round_question::RoundQuestion;
use data::model::quiz::round_category::RoundCategory;
use data::model::quiz::round::Round;
use data::model::quiz::answer::Answer;

type Result<T> = result::Result<T, QuizError>;

pub fn get_question_and_answers(player_id: i32) -> Result<(Question, Vec<Answer>)> {
    let curr_question = get_current_question(player_id)?
        .ok_or_else(|| QuizError::GameAlreadyFinished)?;
    let answers = get_answers(curr_question.id)?;
    Ok((curr_question, answers))
}

pub fn can_use_fifty_fifty_joker(player_id: i32) -> Result<bool> {
    let round_questions = get_round_questions(player_id)?;
    for round_question in round_questions {
        if round_question.is_joker_used {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn use_fifty_fifty_joker(player_id: i32) -> Result<Vec<Answer>> {
    if !can_use_fifty_fifty_joker(player_id)? {
        return Err(QuizError::JokerUnavailable);
    }

    let (_, mut answers) = get_question_and_answers(player_id)?;
    let mut rng = thread_rng();
    rng.shuffle(&mut answers);

    let correct_answer_index = answers.iter()
        .position(|x| x.is_correct)
        .ok_or_else(|| QuizError::GameAlreadyFinished)?;
    let correct_answer = answers.remove(correct_answer_index);
    let mut new_answers = vec![correct_answer];

    let half_len = answers.len() / 2;
    while new_answers.len() < half_len {
        let last_index = answers.len() - 1;
        let last_elem = answers.remove(last_index);
        new_answers.push(last_elem);
    }

    let newest_round = get_current_round_question(player_id)?
        .ok_or_else(|| QuizError::GameAlreadyFinished)?;
    set_joker_user(newest_round.id)?;

    Ok(new_answers)
}

fn get_current_question(player_id: i32) -> Result<Option<Question>> {
    let curr_round = get_current_round(player_id)?
        .ok_or_else(|| QuizError::GameAlreadyFinished)?;
    let newest_question = get_round_questions_joined(curr_round.id)?.remove(0);
    match is_round_finished(newest_question.id)? {
        true => Ok(None),
        false => Ok(Some(newest_question)),
    }
}

fn get_current_round(player_id: i32) -> Result<Option<Round>> {
    let newest_round = get_player_rounds(player_id)?.remove(0);
    match is_round_finished(newest_round.id)? {
        true => Ok(None),
        false => Ok(Some(newest_round)),
    }
}

fn get_current_round_question(player_id: i32) -> Result<Option<RoundQuestion>> {
    let newest_round_question = get_round_questions(player_id)?.remove(0);
    match is_round_finished(newest_round_question.round_id)? {
        true => Ok(None),
        false => Ok(Some(newest_round_question)),
    }
}

fn is_round_finished(round_id: i32) -> Result<bool> {
    let questions = get_round_questions(round_id)?;
    if let Some(answer_id) = questions[0].answer_id {
        let answer = get_answer(answer_id)?;
        return Ok(!answer.is_correct);
    }
    Ok(false)
}


#[derive(Debug)]
pub enum QuizError {
    DatabaseError(DatabaseError),
    JokerUnavailable,
    GameAlreadyFinished,
}

impl fmt::Display for QuizError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QuizError::DatabaseError(ref err) => write!(f, "Database error: {}", err),
            QuizError::JokerUnavailable => write!(f, "Joker error: Tried to use unavailable Joker"),
            QuizError::GameAlreadyFinished => {
                write!(f,
                       "Game already finished error: Tried to interact with a game that has already been finished")
            }
        }
    }
}

impl error::Error for QuizError {
    fn description(&self) -> &str {
        match *self {
            QuizError::DatabaseError(ref err) => err.description(),
            QuizError::JokerUnavailable => "Joker unavailable error",
            QuizError::GameAlreadyFinished => "Game already finished error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            QuizError::DatabaseError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<DatabaseError> for QuizError {
    fn from(err: DatabaseError) -> Self {
        QuizError::DatabaseError(err)
    }
}
