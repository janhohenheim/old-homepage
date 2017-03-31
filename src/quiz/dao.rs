extern crate dotenv;
extern crate diesel;

use self::dotenv::dotenv;
use std::env;
use self::diesel::prelude::*;
use self::diesel::pg::PgConnection;
use super::model::category::*;

type Result<T> = self::diesel::QueryResult<T>;

pub struct Dao {
    conn: PgConnection,
}

impl Dao {
    pub fn new() -> Self {
        Dao { conn: establish_connection() }
    }

    pub fn create_category(&self, text: &str) -> Result<Category> {
        use super::schema::category;
        let new_category = NewCategory { text };
        diesel::insert(&new_category)
            .into(category::table)
            .get_result(&self.conn)
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to database {}",
                                                           database_url))
}
