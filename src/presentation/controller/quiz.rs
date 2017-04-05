extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;
extern crate handlebars_iron as hbs;

use self::iron::{Request, IronResult, Response, status};
use self::iron_sessionstorage::traits::*;
use self::handlebars::to_json;
use presentation::helper::util::{get_formdata, to_ironresult};
use presentation::helper::templating::*;
use presentation::model::section::Section;
use presentation::model::player::Player;
use business::crud::*;

pub fn get_quiz(req: &mut Request) -> IronResult<Response> {
    if get_player_session(req)?.is_some() {
        return get_play(req);
    }
    let template = generate_site_without_data(req, "quiz/quiz_start", Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn post_quiz(req: &mut Request) -> IronResult<Response> {
    if get_player_session(req)?.is_some() || create_player_data(req).is_ok() {
        return post_play(req);
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
    create_player_session(req, new_player.id)
}

pub fn get_admin(req: &mut Request) -> IronResult<Response> {
    let categories = get_categories()
        .unwrap()
        .into_iter()
        .map(|x| x.text)
        .collect::<Vec<String>>();
    let cat_json = btreemap! {
        "categories".to_string() => to_json(&categories),
    };
    let template = generate_site(req, "quiz/admin", cat_json, Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn post_admin(req: &mut Request) -> IronResult<Response> {
    let category = get_formdata(req, "category")?;
    let new_category = create_category(&category);
    to_ironresult(new_category)?;
    get_admin(req)
}

pub fn get_play(req: &mut Request) -> IronResult<Response> {
    if get_player_session(req)?.is_none() {
        return get_quiz(req);
    }
    let template = generate_site_without_data(req, "quiz/quiz_question", Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn post_play(req: &mut Request) -> IronResult<Response> {
    if get_player_session(req)?.is_none() {
        return get_quiz(req);
    }
    let template = generate_site_without_data(req, "quiz/quiz_question", Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}


pub fn get_player_session(req: &mut Request) -> IronResult<Option<Player>> {
    req.session().get::<Player>()
}


pub fn create_player_session(req: &mut Request, id: i32) -> IronResult<()> {
    req.session().set(Player::new(id))
}
