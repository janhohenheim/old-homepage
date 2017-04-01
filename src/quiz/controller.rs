extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;

use session::Player;
use self::iron_sessionstorage::traits::*;
use self::iron::{Request, IronResult, Response, status};
use self::iron::prelude::*;
use std::path::PathBuf;
use super::dao::*;
use self::urlencoded::{UrlEncodedBody, UrlDecodingError};
use std::boxed::Box;
use super::super::templating::*;

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

pub fn get_admin(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(("", status::Ok)))
}

pub fn post_admin(req: &mut Request) -> IronResult<String> {
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
    Ok("quiz/admin".to_string())
}
