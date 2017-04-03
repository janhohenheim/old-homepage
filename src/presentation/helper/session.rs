extern crate iron;
extern crate iron_sessionstorage;
extern crate redis;

use self::iron::prelude::*;
use self::iron_sessionstorage::SessionStorage;
use self::iron_sessionstorage::backends::RedisBackend;
use self::iron_sessionstorage::errors::Error as SessionError;
use self::redis::{IntoConnectionInfo, ConnectionInfo, RedisResult, ConnectionAddr};

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
