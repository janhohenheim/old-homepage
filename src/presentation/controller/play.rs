extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;
extern crate handlebars_iron as hbs;

use self::iron::{Request, IronResult, Response, status};
use presentation::helper::templating::*;
use presentation::model::section::Section;
use presentation::helper::session;
use presentation::helper::util::redirect;

pub fn get_play(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_none() {
        return redirect(req, "get_quiz_start");
    }
    let template = generate_site_without_data(req, "quiz/quiz_question", Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn post_play(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_none() {
        return redirect(req, "get_quiz_start");
    }
    let template = generate_site_without_data(req, "quiz/quiz_question", Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}
