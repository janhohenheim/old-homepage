extern crate diesel;


use self::diesel::prelude::*;
use data::model::quiz::category::*;
use data::model::quiz::player::*;
use data::establish_connection;
use data::schema;
use self::diesel::result::{Error, DatabaseErrorKind};

type Result<T> = self::diesel::QueryResult<T>;

pub fn create_player(name: &str) -> Result<Player> {
    if name.is_empty() {
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Name cannot be empty".to_owned())));
    }
    use self::schema::player;
    let new_player = NewPlayer { name };
    let conn = establish_connection();
    diesel::insert(&new_player)
        .into(player::table)
        .get_result(&conn)
}

pub fn create_category(cat_text: &str) -> Result<Category> {
    if cat_text.is_empty() {
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Text cannot be empty".to_owned())));
    }
    use self::schema::category;
    use self::schema::category::dsl::*;
    let conn = establish_connection();
    let already_created_cats = category.filter(text.like(cat_text))
        .load::<Category>(&conn)?;
    if !already_created_cats.is_empty() {
        return activate_category(already_created_cats[0].id);
    }

    let new_category = NewCategory { text: cat_text };
    diesel::insert(&new_category)
        .into(category::table)
        .get_result(&conn)
}

pub fn get_categories() -> Result<Vec<Category>> {
    use self::schema::category::dsl::*;
    let conn = establish_connection();
    category
        .filter(is_active.eq(true))
        .order(text.asc())
        .load::<Category>(&conn)
}

pub fn rename_category(cat_id: i32, cat_text: &str) -> Result<Category> {
    use self::schema::category::dsl::*;
    let conn = establish_connection();
    diesel::update(category.find(cat_id))
        .set(text.eq(cat_text))
        .get_result::<Category>(&conn)
}

pub fn deactivate_category(cat_id: i32) -> Result<Category> {
    set_category_active_state(cat_id, false)
}

fn activate_category(cat_id: i32) -> Result<Category> {
    set_category_active_state(cat_id, true)
}

fn set_category_active_state(cat_id: i32, state: bool) -> Result<Category> {
    use self::schema::category::dsl::*;
    let conn = establish_connection();
    diesel::update(category.find(cat_id))
        .set(is_active.eq(state))
        .get_result::<Category>(&conn)
}
