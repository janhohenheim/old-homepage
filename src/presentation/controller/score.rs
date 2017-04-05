extern crate iron;
extern crate router;

use self::iron::{Request, IronResult, Response, status};
use presentation::helper::templating::*;

pub fn get_score(req: &mut Request) -> IronResult<Response> {
    let template = generate_site_without_data(req, "quiz/scoreboard", Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}
