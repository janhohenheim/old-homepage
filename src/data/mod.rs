extern crate diesel;
extern crate dotenv;

use std::env;
use self::dotenv::dotenv;
use self::diesel::prelude::*;
use self::diesel::pg::PgConnection;

pub mod schema;
pub mod model;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to database {}",
                                                           database_url))
}
