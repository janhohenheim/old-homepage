extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;
extern crate handlebars_iron as hbs;

use self::iron::{Request, IronResult, Response, status};
use presentation::helper::templating::*;
use presentation::helper::session;
use presentation::helper::util::redirect;


pub fn get_dashboard(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_none() {
        return redirect(req, "get_root");
    }
    let template = generate_site_without_data(req, "quiz/admin/start", None);
    Ok(Response::with((template, status::Ok)))
}
