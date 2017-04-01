extern crate iron;
extern crate iron_sessionstorage;
extern crate serde;
extern crate serde_json;
extern crate redis;

use self::iron::prelude::*;
use self::iron_sessionstorage::traits::*;
use self::iron_sessionstorage::SessionStorage;
use self::iron_sessionstorage::backends::RedisBackend;
use self::iron_sessionstorage::errors::Error as SessionError;
use self::redis::{IntoConnectionInfo, ConnectionInfo, RedisResult, ConnectionAddr};
use self::serde_json::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Player {
    pub id: i32,
    pub selected_answer_index: Option<i32>,
}

impl Player {
    pub fn new(id: i32) -> Self {
        Player{
            id,
            selected_answer_index: None,
        }
    }
}

impl iron_sessionstorage::Value for Player {
    fn get_key() -> &'static str {
        "player"
    }
    fn into_raw(self) -> String {
        to_string(&self).unwrap()
    }
    fn from_raw(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            from_str(&value).ok()
        }
    }
}

pub fn get_player(req: &mut Request) -> IronResult<Option<Player>> {
    req.session().get::<Player>()
}


pub fn create_player(req: &mut Request, id: i32) -> IronResult<()> {
    req.session().set(Player::new(id))
}


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
