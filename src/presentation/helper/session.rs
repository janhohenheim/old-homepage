extern crate iron;
extern crate iron_sessionstorage;
extern crate redis;

use self::iron::prelude::*;
use self::iron_sessionstorage as iss;
use self::iss::SessionStorage;
use self::iss::backends::RedisBackend;
use self::iss::errors::Error as SessionError;
use self::iss::traits::*;
use self::redis::{IntoConnectionInfo, ConnectionInfo, RedisResult, ConnectionAddr};
use presentation::model::player::Player;
use presentation::model::admin::Admin;


pub fn link_to_chain(chain: &mut Chain) -> Result<&mut Chain, SessionError> {
    let backend = RedisBackend::new(RedisConnection)?;
    let session_storage = SessionStorage::new(backend);
    Ok(chain.link_around(session_storage))
}

struct RedisConnection;
impl IntoConnectionInfo for RedisConnection {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo> {
        let addr = ConnectionAddr::Tcp("127.0.0.1".to_string(), 6379);
        let connection = ConnectionInfo {
            // A boxed connection address for where to connect to.
            addr: Box::new(addr),
            // The database number to use.  This is usually `0`.
            db: 0,
            // Optionally a password that should be used for connection.
            passwd: (None),
        };
        Ok(connection)
    }
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
