extern crate iron;
extern crate iron_sessionstorage;
extern crate dotenv;

use std::env;
use self::dotenv::dotenv;
use self::iron::prelude::*;
use self::iron_sessionstorage as iss;
use self::iss::SessionStorage;
use self::iss::backends::SignedCookieBackend;
use self::iss::errors::Error as SessionError;
use self::iss::traits::*;
use presentation::model::player::Player;
use presentation::model::admin::Admin;


pub fn link_to_chain(chain: &mut Chain) -> Result<&mut Chain, SessionError> {
    dotenv().ok();

    let signature = env::var("COOKIE_SIGNATURE").expect("Secret cookie signature must be set");
    let my_secret = signature.as_bytes().to_vec();
    Ok(chain.link_around(SessionStorage::new(SignedCookieBackend::new(my_secret))))
}

pub fn clear(req: &mut Request) -> IronResult<()> {
    req.session().clear()
}

pub fn get_player(req: &mut Request) -> IronResult<Option<Player>> {
    req.session().get::<Player>()
}


pub fn create_player(req: &mut Request, id: i32) -> IronResult<()> {
    req.session().set(Player::new(id))
}


pub fn get_admin(req: &mut Request) -> IronResult<Option<Admin>> {
    req.session().get::<Admin>()
}


pub fn create_admin(req: &mut Request, id: i32, name: &str) -> IronResult<()> {
    req.session().set(Admin::new(id, name.to_owned()))
}
