extern crate iron;
extern crate iron_sessionstorage;

use self::iron_sessionstorage::traits::*;
use self::iron::{Request, Response, IronResult, status};
use presentation::model::admin::Admin;
use presentation::helper::util::get_formdata;

pub fn post_login(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}

/*
pub fn login(email: String, password: String) -> Option<UserAccount> {
    if &email == "test" && &password == "test" {
        return Some(UserAccount {
                        id: 0,
                        email: "foo@bar.com".to_owned(),
                        name: "Foo".to_owned(),
                        password: "test".to_owned(),
                    });
    }
    return None;
}
*/
pub fn get_admin(req: &mut Request) -> IronResult<Option<Admin>> {
    req.session().get::<Admin>()
}


pub fn create_admin(req: &mut Request, id: i32, name: &str) -> IronResult<()> {
    req.session().set(Admin::new(id, name.to_owned()))
}
