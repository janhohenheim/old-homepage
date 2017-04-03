extern crate iron;

use presentation::helper::util::get_formdata;
use self::iron::{Request, Response, IronResult, status};

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