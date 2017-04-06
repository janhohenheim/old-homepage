extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;
extern crate handlebars_iron as hbs;

use self::iron::{Request, IronResult, Response, status};
use self::handlebars::to_json;

use presentation::helper::util::{get_formdata, to_ironresult, redirect};
use presentation::helper::templating::*;
use presentation::helper::session;
use presentation::model::section::Section;
use business::crud::*;

pub fn get_quiz(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_some() {
        return redirect(req, "get_quiz_play");
    }
    let template = generate_site_without_data(req, "quiz/quiz_start", Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn post_quiz(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_some() || create_player_data(req).is_ok() {
        return redirect(req, "get_quiz_play");
    }

    let err = btreemap!{
        "error".to_string() => to_json(&"true".to_string())
    };
    let template = generate_site(req, "quiz/quiz_start", err, Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

fn create_player_data(req: &mut Request) -> IronResult<()> {
    let name = get_formdata(req, "name")?;
    let new_player = create_player(&name);
    let new_player = to_ironresult(new_player)?;
    session::create_player(req, new_player.id)
}
