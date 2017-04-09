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
use business::crud::*;


pub fn get_play(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_none() {
        return redirect(req, "get_quiz_start");
    }



    let answer0 = Answer {
        id: 1,
        text: "Bar".to_owned(),
        is_confirmed_wrong: false,
    };
    let answer1 = Answer {
        id: 2,
        text: "Baz".to_owned(),
        is_confirmed_wrong: true,
    };
    let answer2 = Answer {
        id: 3,
        text: "Quux".to_owned(),
        is_confirmed_wrong: true,
    };
    let answer3 = Answer {
        id: 4,
        text: "Memes".to_owned(),
        is_confirmed_wrong: false,
    };

    let dummy = AskedQuestion {
        text: "Foo?".to_owned(),
        answers: vec![answer0, answer1, answer2, answer3],
    };

    let data = btreemap!{
        "question".to_string() => to_json(&dummy),
    };

    let cats = to_ironresult(get_round_categories_joined(1))?;
    println!("{:?}", cats);

    let template = generate_site(req, "quiz/quiz_question", data, Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn post_play(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_none() {
        return redirect(req, "get_quiz_start");
    }

    redirect(req, "get_quiz_play")
}
