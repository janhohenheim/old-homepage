extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;
extern crate handlebars_iron as hbs;

use self::iron::{Request, IronResult, Response, status};
use presentation::helper::templating::*;
use presentation::model::section::Section;
use self::iron::modifiers::Redirect;

use presentation::helper::session::{get_player_session};

pub fn get_play(req: &mut Request) -> IronResult<Response> {
    if get_player_session(req)?.is_none() {
        return redirect_to_start(req);
    }
    let template = generate_site_without_data(req, "quiz/quiz_question", Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn post_play(req: &mut Request) -> IronResult<Response> {
    if get_player_session(req)?.is_none() {
        return redirect_to_start(req);
    }
    let template = generate_site_without_data(req, "quiz/quiz_question", Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

fn redirect_to_start(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Found, Redirect(url_for!(req, "get_quiz")))))
}