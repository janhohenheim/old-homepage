extern crate diesel;


use self::diesel::prelude::*;
use data::model::quiz::category::*;
use data::model::quiz::player::*;
use data::establish_connection;
use data::schema;

type Result<T> = self::diesel::QueryResult<T>;

pub fn create_player(name: &str) -> Result<Player> {
    use self::schema::player;
    let new_player = NewPlayer { name };
    let conn = establish_connection();
    diesel::insert(&new_player)
        .into(player::table)
        .get_result(&conn)
}

pub fn create_category(text: &str) -> Result<Category> {
    use self::schema::category;
    let new_category = NewCategory { text };
    let conn = establish_connection();
    diesel::insert(&new_category)
        .into(category::table)
        .get_result(&conn)
}

pub fn get_categories() -> Result<Vec<Category>> {
    use self::schema::category::dsl::*;
    let conn = establish_connection();
    category
        .filter(is_active.eq(true))
        .load::<Category>(&conn)
}
