extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;
extern crate handlebars_iron as hbs;

use self::iron::{Request, IronResult, Response, status};
use self::handlebars::to_json;

use presentation::helper::templating::*;
use presentation::model::section::Section;
use presentation::model::asked_question::AskedQuestion;
use presentation::model::answer::Answer;
use presentation::helper::session;
use presentation::helper::util::{redirect, to_ironresult, get_formdata};
use business::quiz::*;
use business::quiz_error::QuizError;

pub fn get_play(req: &mut Request) -> IronResult<Response> {
    let pl = session::get_player(req)?;
    if pl.is_none() {
        return redirect(req, "get_quiz_start");
    }
    let player = pl.unwrap();

    if !to_ironresult(is_game_in_progress(player.id))? {
        session::clear(req)?;
        return redirect(req, "get_quiz_score");
    }

    let (question, answers) = to_ironresult(get_question_and_answers(player.id))?;
    let presentation_answers = answers
        .into_iter()
        .map(|x| {
                 Answer {
                     id: x.id,
                     text: x.text,
                     is_confirmed_wrong: false,
                 }
             })
        .collect::<Vec<Answer>>();
    let asked_question = AskedQuestion {
        text: question.text,
        answers: presentation_answers,
    };

    let data = btreemap!{
        "question".to_string() => to_json(&asked_question),
    };
    let template = generate_site(req, "quiz/quiz_question", data, Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn post_play(req: &mut Request) -> IronResult<Response> {
    let pl = session::get_player(req)?;
    if pl.is_none() {
        return redirect(req, "get_quiz_start");
    }
    let player = pl.unwrap();
    if !to_ironresult(is_game_in_progress(player.id))? {
        return redirect(req, "get_quiz_start");
    }
    let ans = get_formdata(req, "answer")?;
    if ans.is_empty() {
        return redirect(req, "get_quiz_play");
    }
    let answer_id = to_ironresult(ans.parse::<i32>())?;
    let was_correct = answer(player.id, answer_id);
    if let &Err(ref err) = &was_correct {
        if let &QuizError::OutOfResources = err {
            return redirect(req, "get_quiz_score");
        }
    }
    match to_ironresult(was_correct)? {
        AnswerResult::Correct => redirect(req, "get_quiz_play"),
        AnswerResult::Wrong(_) => redirect(req, "get_quiz_score"),
    }
}

pub fn post_joker(req: &mut Request) -> IronResult<Response> {
    let player = session::get_player(req)?;
    if player.is_none() {
        return redirect(req, "get_quiz_start");
    }
    let player_id = player.unwrap().id;
    if !to_ironresult(is_game_in_progress(player_id))? {
        return redirect(req, "get_quiz_start");
    }

    if to_ironresult(can_use_fifty_fifty_joker(player_id))? {
        to_ironresult(use_fifty_fifty_joker(player_id))?;
    }
    redirect(req, "get_quiz_play")
}

pub fn post_finish(req: &mut Request) -> IronResult<Response> {
    let player = session::get_player(req)?;
    if player.is_none() {
        return redirect(req, "get_quiz_start");
    }
    let player_id = player.unwrap().id;
    if !to_ironresult(is_game_in_progress(player_id))? {
        return redirect(req, "get_quiz_start");
    }
    to_ironresult(finish_game(player_id))?;
    session::clear(req)?;
    redirect(req, "get_quiz_score")
}
