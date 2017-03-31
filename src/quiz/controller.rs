extern crate iron;
extern crate iron_sessionstorage;

use session::Player;
use self::iron_sessionstorage::traits::*;
use self::iron::{Request, IronResult};
use std::path::PathBuf;


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

