extern crate iron;
extern crate iron_sessionstorage;

use self::iron_sessionstorage::traits::*;
use self::iron::{Request, Response, IronResult, status};
use presentation::model::admin::Admin;
use presentation::helper::util::{get_formdata, to_ironresult};
use business::login::login;

pub fn post_login(req: &mut Request) -> IronResult<Response> {
    let content_type = mime!(Text / Html);
    let go_back =
        Ok(Response::with((content_type, status::Ok, "<script>history.go(-1);</script>")));

    if get_admin(req)?.is_some() {
        return go_back;
    }
    let email = get_formdata(req, "email")?;
    let pwd = get_formdata(req, "password")?;
    if let Some(user) = to_ironresult(login(&email, &pwd))? {
        create_admin(req, user.id, &user.name)?;
        return go_back;
    }

    //TODO: Show message for "invalid login"
    go_back
}

pub fn get_admin(req: &mut Request) -> IronResult<Option<Admin>> {
    req.session().get::<Admin>()
}


pub fn create_admin(req: &mut Request, id: i32, name: &str) -> IronResult<()> {
    req.session().set(Admin::new(id, name.to_owned()))
}
