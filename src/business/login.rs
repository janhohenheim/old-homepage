extern crate diesel;

use self::diesel::prelude::*;
use data::schema;
use data::model::user_account::*;
use data::establish_connection;
use super::crypto;

type LoginResult<T> = self::diesel::QueryResult<T>;

pub fn is_login_correct(email: &str, password: &str) {}

pub fn register(email: &str, name: &str, password: &str) -> LoginResult<UserAccount> {
    use self::schema::user_account;

    let encrypted_password = crypto::encrypt(password);
    let new_user = NewUserAccount {
        email: email.to_owned(),
        name: name.to_owned(),
        password: encrypted_password,
    };

    let conn = establish_connection();
    diesel::insert(&new_user)
        .into(user_account::table)
        .get_result(&conn)
}
