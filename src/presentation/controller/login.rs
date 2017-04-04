extern crate iron;
extern crate iron_sessionstorage;

use self::iron_sessionstorage::traits::*;
use self::iron::{Request, Response, IronResult, status};
use self::iron::modifiers::Redirect;
use presentation::model::admin::Admin;
use presentation::helper::util::{get_formdata, to_ironresult};
use business::login::{login, register};

pub fn post_login(req: &mut Request) -> IronResult<Response> {
    if get_admin(req)?.is_some() {
        return Ok(get_to_root(req));
    }
    let email = get_formdata(req, "email")?;
    let pwd = get_formdata(req, "password")?;
    if let Some(user) = to_ironresult(login(&email, &pwd))? {
        create_admin(req, user.id, &user.name)?;
        return Ok(get_to_root(req));
    }

    //TODO: Show message for "invalid login"
    Ok(get_to_root(req))
}

pub fn post_register(req: &mut Request) -> IronResult<Response> {
    if get_admin(req)?.is_some() {
        return Ok(get_to_root(req));
    }

    let email = get_formdata(req, "email")?;
    let pwd = get_formdata(req, "password")?;
<<<<<<< HEAD
    if !email.is_empty() && pwd.len() >= 8 {
        if let Ok(user) = to_ironresult(register(&email, &email, &pwd)) {
            create_admin(req, user.id, &user.name)?;
            return Ok(get_to_root(req));
        }
    }
=======
    if let Ok(user) = to_ironresult(register(&email, &email, &pwd)) {
        create_admin(req, user.id, &user.name)?;
        return Ok(get_to_root(req));
    }

>>>>>>> 8e4416c8d1345c80710d9f5fffda5e111a4ab0ed
    //TODO: Show message for "invalid register"
    Ok(get_to_root(req))
}

pub fn get_logout(req: &mut Request) -> IronResult<Response> {
    req.session().clear()?;
    Ok(get_to_root(req))
}

fn get_to_root(req: &mut Request) -> Response {
    Response::with((status::Found, Redirect(url_for!(req, "get_root"))))
}

pub fn get_admin(req: &mut Request) -> IronResult<Option<Admin>> {
    req.session().get::<Admin>()
}


pub fn create_admin(req: &mut Request, id: i32, name: &str) -> IronResult<()> {
    req.session().set(Admin::new(id, name.to_owned()))
}
