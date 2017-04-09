extern crate rand;

use self::rand::{thread_rng, Rng};

use std::vec::Vec;
use std::result;

use super::crud::*;
use super::quiz_error::QuizError;

use data::model::quiz::question::Question;
use data::model::quiz::round_question::RoundQuestion;
use data::model::quiz::round::Round;
use data::model::quiz::answer::Answer;

type Result<T> = result::Result<T, QuizError>;


pub fn is_game_in_progress(player_id: i32) -> Result<bool> {
    let round = get_last_round(player_id)?;
    if let Some(rnd) = round {
        return Ok(!rnd.is_finished);
    }
    Ok(false)
}

pub fn get_question_and_answers(player_id: i32) -> Result<(Question, Vec<Answer>)> {
    let curr_question = get_current_question(player_id)?
        .ok_or_else(|| QuizError::GameAlreadyFinished)?;
    let answers = get_answers(curr_question.id)?;
    Ok((curr_question, answers))
}

pub fn answer(player_id: i32, answer_id: i32) -> Result<AnswerResult> {
    is_answer_valid(player_id, answer_id)?;
    let round_question = get_current_round_question(player_id)?
        .ok_or_else(|| QuizError::GameAlreadyFinished)?;
    set_end_time_to_now(round_question.id)?;
    set_answer(round_question.id, answer_id)?;
    let answer = get_answer(answer_id)?;
    if !answer.is_correct {
        let (_, answers) = get_question_and_answers(player_id)?;
        finish_game(player_id)?;
        for ans in answers {
            if ans.is_correct {
                return Ok(AnswerResult::Wrong(ans));
            }
        }
        return Err(QuizError::StateError);
    }

    let result = generate_new_round_question(player_id);
    if let Err(err) = result {
        if let QuizError::OutOfResources = err {
            finish_game(player_id)?;
        } else {
            return Err(err);
        }
    }

    Ok(AnswerResult::Correct)
}

pub enum AnswerResult {
    Correct,
    Wrong(Answer),
}


pub fn start_game(player_id: i32, category_ids: Vec<i32>) -> Result<(Question, Vec<Answer>)> {
    if is_game_in_progress(player_id)? {
        return Err(QuizError::GameStillInProgress);
    }
    let round = create_round(player_id)?;
    for category_id in category_ids {
        create_round_category(round.id, category_id)?;
    }
    generate_new_round_question(player_id)?;
    get_question_and_answers(player_id)
}

pub fn finish_game(player_id: i32) -> Result<Round> {
    if !is_game_in_progress(player_id)? {
        return Err(QuizError::NoGameInProgress);
    }

    let round_question = get_current_round_question(player_id)?
        .ok_or_else(|| QuizError::StateError)?;
    set_end_time_to_now(round_question.id)?;
    let round = get_last_round(player_id)?.unwrap();
    set_round_finished(round.id)?;
    Ok(round)
}

pub fn can_use_fifty_fifty_joker(player_id: i32) -> Result<bool> {
    let curr_round = get_last_round(player_id)?
        .ok_or_else(|| QuizError::NoGameInProgress)?;
    let round_questions = get_round_questions(curr_round.id)?;
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
        .ok_or_else(|| QuizError::StateError)?;
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

fn is_answer_valid(player_id: i32, answer_id: i32) -> Result<bool> {
    let question = get_current_question(player_id)?
        .ok_or_else(|| QuizError::GameAlreadyFinished)?;
    let answers = get_answers(question.id)?;
    Ok(answers.iter().any(|x| x.id == answer_id && x.is_active))
}

fn generate_new_round_question(player_id: i32) -> Result<RoundQuestion> {
    let round = get_last_round(player_id)?
        .ok_or_else(|| QuizError::NoGameInProgress)?;

    if is_round_finished(round.id)? {
        return Err(QuizError::NoGameInProgress);
    }
    let mut categories = get_round_categories_joined(round.id)?;
    let mut rng = thread_rng();
    rng.shuffle(&mut categories);
    for category in categories {
        let mut questions = get_questions_with_category(category.id)?;
        rng.shuffle(&mut questions);
        let round_questions = get_round_questions(round.id)?;
        for question in questions {
            if !round_questions
                    .iter()
                    .any(|x| x.question_id == question.id) {
                let new_round_question = create_round_question(round.id, question.id)?;
                return Ok(new_round_question);
            }
        }
    }
    Err(QuizError::OutOfResources)
}

fn get_current_question(player_id: i32) -> Result<Option<Question>> {
    if !is_game_in_progress(player_id)? {
        return Err(QuizError::NoGameInProgress);
    }
    let round = get_last_round(player_id)?.unwrap();
    let mut questions = get_round_questions_joined(round.id)?;
    if questions.is_empty() {
        return Ok(None);
    }
    Ok(Some(questions.remove(0)))
}

fn get_last_round(player_id: i32) -> Result<Option<Round>> {
    let mut rounds = get_player_rounds(player_id)?;
    if rounds.is_empty() {
        return Ok(None);
    }
    Ok(Some(rounds.remove(0)))
}

fn get_current_round_question(player_id: i32) -> Result<Option<RoundQuestion>> {
    if !is_game_in_progress(player_id)? {
        return Err(QuizError::NoGameInProgress);
    }
    let round = get_last_round(player_id)?.unwrap();
    let mut round_questions = get_round_questions(round.id)?;
    if round_questions.is_empty() {
        return Ok(None);
    }
    Ok(Some(round_questions.remove(0)))
}

fn is_round_finished(round_id: i32) -> Result<bool> {
    let round = get_round(round_id)?;
    Ok(round.is_finished)
}
