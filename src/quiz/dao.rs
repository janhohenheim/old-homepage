extern crate dotenv;
extern crate diesel;

use self::dotenv::dotenv;
use std::env;
use super::model::Player;
use self::diesel::prelude::*;
use self::diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn generate_player_id() -> u32 {
    unimplemented!();
    0
}

pub fn get_player(id: u32) -> Player {
    unimplemented!();
    Player {}
}
