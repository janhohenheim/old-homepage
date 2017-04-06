extern crate iron;
extern crate iron_sessionstorage;

use self::iron::{Request, Response, IronResult, status};
use self::iron::modifiers::Redirect;
use presentation::helper::session;
use presentation::helper::util::{get_formdata, to_ironresult};
use business::login::{login, register};

pub fn post_login(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_some() {
        return Ok(get_to_root(req));
    }
    let email = get_formdata(req, "email")?;
    let pwd = get_formdata(req, "password")?;
    if let Some(user) = to_ironresult(login(&email, &pwd))? {
        session::create_admin(req, user.id, &user.name)?;
        return Ok(get_to_root(req));
    }

    //TODO: Show message for "invalid login"
    Ok(get_to_root(req))
}

pub fn post_register(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_some() {
        return Ok(get_to_root(req));
    }

    let email = get_formdata(req, "email")?;
    let pwd = get_formdata(req, "password")?;
    if !email.is_empty() && pwd.len() >= 8 {
        if let Ok(user) = to_ironresult(register(&email, &email, &pwd)) {
            session::create_admin(req, user.id, &user.name)?;
            return Ok(get_to_root(req));
        }
    }
    //TODO: Show message for "invalid register"
    Ok(get_to_root(req))
}

pub fn get_logout(req: &mut Request) -> IronResult<Response> {
    session::clear(req)?;
    Ok(get_to_root(req))
}

fn get_to_root(req: &mut Request) -> Response {
    Response::with((status::Found, Redirect(url_for!(req, "get_root"))))
}
