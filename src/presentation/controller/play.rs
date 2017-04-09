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
use presentation::helper::util::{redirect, to_ironresult};
use business::quiz::*;

pub fn get_play(req: &mut Request) -> IronResult<Response> {
    let pl = session::get_player(req)?;
    if pl.is_none() {
        return redirect(req, "get_quiz_start");
    }
    let player = pl.unwrap();

    let (question, answers) = match to_ironresult(is_game_in_progress(player.id))? {
        true => to_ironresult(get_question_and_answers(player.id))?,
        false => {
            to_ironresult(start_game(player.id,
                                     player
                                         .categories
                                         .iter()
                                         .map(|x| x.id)
                                         .collect::<Vec<i32>>()))?
        }
    };

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
    if session::get_player(req)?.is_none() {
        return redirect(req, "get_quiz_start");
    }
    redirect(req, "get_quiz_play")
}
