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


pub fn start(req: &mut Request) -> IronResult<PathBuf> {
    let player = req.session().get::<Player>()?;
    match player {
        Some(_) => Ok(PathBuf::from("quiz/quiz_question.hbs")),
        None => Ok(PathBuf::from("quiz/quiz_start.hbs")),
    }
}

pub fn start_post(req: &mut Request) -> IronResult<PathBuf> {
    let player = req.session().get::<Player>()?;
    if player.is_none() {
        req.session().set(Player { id: 0 })?;
    }
    Ok(PathBuf::from("quiz/quiz_question.hbs"))
}


pub fn admin_post(req: &mut Request) -> IronResult<PathBuf> {
    let category = {
        let formdata = req.get_ref::<UrlEncodedBody>().map_err(|err| IronError{ error: Box::new(err), response: Response::with(status::BadRequest) })?;
        let categories = formdata.get("category").ok_or(IronError{ error: (Box::new(UrlDecodingError::EmptyQuery)), response: Response::with(status::BadRequest) })?;
        categories[0].to_owned()
    };
    let dao = Dao::new();
    dao.create_category(&category).map_err(|err| IronError{ error: Box::new(err), response: Response::with(status::BadRequest) })?;
    Ok(PathBuf::from("quiz/admin.hbs"))
}