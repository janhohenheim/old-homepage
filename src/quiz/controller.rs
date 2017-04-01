extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;

use session::Player;
use self::iron_sessionstorage::traits::*;
use self::iron::{Request, IronResult, Response, status};
use self::iron::prelude::*;
use super::dao::*;
use self::urlencoded::{UrlEncodedBody, UrlDecodingError};
use std::boxed::Box;
use super::super::templating::*;
use self::handlebars::to_json;

pub fn get_start(req: &mut Request) -> IronResult<String> {
    let player = req.session().get::<Player>()?;
    match player {
        Some(_) => Ok("quiz/quiz_question".to_string()),
        None => Ok("quiz/quiz_start".to_string()),
    }
}

pub fn post_start(req: &mut Request) -> IronResult<String> {
    let player = req.session().get::<Player>()?;
    if player.is_none() {
        req.session().set(Player { id: 0 })?;
    }
    Ok("quiz/quiz_question".to_string())
}

pub fn get_admin(_: &mut Request) -> IronResult<Response> {
    let dao = Dao::new();
    let categories = dao.get_categories()
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
    let category = {
        let formdata = req.get_ref::<UrlEncodedBody>()
            .map_err(|err| {
                         IronError {
                             error: Box::new(err),
                             response: Response::with(status::BadRequest),
                         }
                     })?;
        let categories = formdata.get("category")
            .ok_or(IronError {
                       error: (Box::new(UrlDecodingError::EmptyQuery)),
                       response: Response::with(status::BadRequest),
                   })?;
        categories[0].to_owned()
    };
    let dao = Dao::new();
    dao.create_category(&category)
        .map_err(|err| {
                     IronError {
                         error: Box::new(err),
                         response: Response::with(status::BadRequest),
                     }
                 })?;
    get_admin(req)
}
