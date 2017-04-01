extern crate dotenv;
extern crate diesel;

use self::dotenv::dotenv;
use std::env;
use self::diesel::prelude::*;
use self::diesel::pg::PgConnection;
use super::model::category::*;
use super::model::player::*;

type Result<T> = self::diesel::QueryResult<T>;

pub fn create_player(name: &str) -> Result<Player> {
    use super::schema::player;
    let new_player = NewPlayer { name };
    let conn = establish_connection();
    diesel::insert(&new_player)
        .into(player::table)
        .get_result(&conn)
}

pub fn create_category(text: &str) -> Result<Category> {
    use super::schema::category;
    let new_category = NewCategory { text };
    let conn = establish_connection();
    diesel::insert(&new_category)
        .into(category::table)
        .get_result(&conn)
}

pub fn get_categories() -> Result<Vec<Category>> {
    use super::schema::category::dsl::*;
    let conn = establish_connection();
    category
        .filter(is_active.eq(true))
        .load::<Category>(&conn)
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to database {}",
                                                           database_url))
}
