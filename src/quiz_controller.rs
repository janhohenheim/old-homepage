extern crate iron;
extern crate iron_sessionstorage;

use session::Guest;
use self::iron_sessionstorage::traits::*;
use self::iron::{Request, IronResult};
use std::path::PathBuf;

pub fn start(req: &mut Request) -> IronResult<PathBuf> {
    let session = req.session().get::<Guest>()?;
    if session.is_none() {
        req.session().set(Guest)?;
    }
    Ok(PathBuf::from("quiz/quiz_question.hbs"))
}