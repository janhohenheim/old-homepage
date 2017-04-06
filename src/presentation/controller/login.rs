extern crate iron;
extern crate iron_sessionstorage;

use self::iron::{Request, Response, IronResult};
use presentation::helper::session;
use presentation::helper::util::{get_formdata, to_ironresult, redirect};
use business::login::{login, register};

pub fn post_login(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_some() {
        return redirect(req, "get_root");
    }
    let email = get_formdata(req, "email")?;
    let pwd = get_formdata(req, "password")?;
    if let Some(user) = to_ironresult(login(&email, &pwd))? {
        session::create_admin(req, user.id, &user.name)?;
        return redirect(req, "get_root");
    }

    //TODO: Show message for "invalid login"
    redirect(req, "get_root")
}

pub fn post_register(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_some() {
        return redirect(req, "get_root");
    }

    let email = get_formdata(req, "email")?;
    let pwd = get_formdata(req, "password")?;
    if !email.is_empty() && pwd.len() >= 8 {
        if let Ok(user) = to_ironresult(register(&email, &email, &pwd)) {
            session::create_admin(req, user.id, &user.name)?;
            return redirect(req, "get_root");
        }
    }
    //TODO: Show message for "invalid register"
    redirect(req, "get_root")
}

pub fn get_logout(req: &mut Request) -> IronResult<Response> {
    session::clear(req)?;
    redirect(req, "get_root")
}
