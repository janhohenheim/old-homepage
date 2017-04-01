extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;

use session;
use self::iron::{Request, IronResult, Response, status};
use self::iron::prelude::*;
use super::dao::*;
use self::urlencoded::{UrlEncodedBody, UrlDecodingError};
use std::boxed::Box;
use super::super::templating::*;
use self::handlebars::to_json;
use std::error::Error;

pub fn get_start(req: &mut Request) -> IronResult<String> {
    let player = session::get_player(req)?;
    match player {
        Some(_) => Ok("quiz/quiz_question".to_string()),
        None => Ok("quiz/quiz_start".to_string()),
    }
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

pub fn post_start(req: &mut Request) -> IronResult<String> {
    let player = session::get_player(req)?;
    if player.is_none() {
        let new_player = create_player("adolfo");
        let new_player = to_ironresult(new_player)?;
        session::create_player(req, session::Player { id: new_player.id })?
    }
    Ok("quiz/quiz_question".to_string())
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
    let category = {
        let formdata = req.get_ref::<UrlEncodedBody>();
        let formdata = to_ironresult(formdata)?;
        let categories = formdata.get("category")
            .ok_or(IronError {
                       error: (Box::new(UrlDecodingError::EmptyQuery)),
                       response: Response::with(status::BadRequest),
                   })?;
        categories[0].to_owned()
    };
    let new_category = create_category(&category);
    to_ironresult(new_category)?;
    get_admin(req)
}
