extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;
extern crate handlebars_iron as hbs;

use session;
use self::iron::{Request, IronResult, Response, status};
use self::iron::prelude::*;
use super::dao::*;
use self::urlencoded::{UrlEncodedBody, UrlDecodingError};
use std::boxed::Box;
use super::super::templating::*;
use self::handlebars::to_json;
use std::error::Error;

pub fn get_quiz(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_some() {
        return get_play(req);
    }
    let template = generate_site_without_data("quiz/quiz_start", Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn post_quiz(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_some() || create_player_data(req).is_ok() {
        return post_play(req);
    }

    let err = btreemap!{
        "error".to_string() => to_json(&"true".to_string())
    };
    let template = generate_site("quiz/quiz_start", err, Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn get_admin(_: &mut Request) -> IronResult<Response> {
    let categories = get_categories()
        .unwrap()
        .into_iter()
        .map(|x| x.text)
        .collect::<Vec<String>>();
    let cat_json = btreemap! {
        "categories".to_string() => to_json(&categories),
    };
    let template = generate_site("quiz/admin", cat_json, Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn post_admin(req: &mut Request) -> IronResult<Response> {
    let category = get_formdata(req, "category")?;
    let new_category = create_category(&category);
    to_ironresult(new_category)?;
    get_admin(req)
}

pub fn get_play(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_none() {
        return get_quiz(req);
    }
    let template = generate_site_without_data("quiz/quiz_question", Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn post_play(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_none() {
        return get_quiz(req);
    }
    let template = generate_site_without_data("quiz/quiz_question", Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

fn create_player_data(req: &mut Request) -> IronResult<()> {
    let name = get_formdata(req, "name")?;
    let new_player = create_player(&name);
    let new_player = to_ironresult(new_player)?;
    session::create_player(req, new_player.id)
}

fn to_ironresult<T, E>(result: Result<T, E>) -> IronResult<T>
    where E: Send + Error + 'static
{
    result.map_err(|err| {
                       IronError {
                           error: Box::new(err),
                           response: Response::with(status::BadRequest),
                       }
                   })
}

fn get_formdata(req: &mut Request, form_id: &str) -> IronResult<String> {
    let formdata = req.get_ref::<UrlEncodedBody>();
    let formdata = to_ironresult(formdata)?;
    let data = formdata.get(form_id)
        .ok_or(IronError {
                   error: (Box::new(UrlDecodingError::EmptyQuery)),
                   response: Response::with(status::BadRequest),
               })?;
    Ok(data[0].to_owned())
}
