extern crate diesel;

use self::diesel::prelude::*;
use data::schema;
use data::model::user_account::*;
use data::establish_connection;
use super::crypto::{encrypt, check};

type LoginResult<T> = self::diesel::QueryResult<T>;

pub fn login(user_email: &str, pwd: &str) -> LoginResult<Option<UserAccount>> {
    use self::schema::user_account::dsl::*;
    let conn = establish_connection();

    let mut found_users = user_account.filter(email.eq(user_email)).load::<UserAccount>(&conn)?;

    if found_users.is_empty() {
        return Ok(None);
    }

    let found_user = found_users.remove(0);
    let is_correct_pwd = check(pwd, &found_user.password);

    return match is_correct_pwd {
               true => Ok(Some(found_user)),
               false => Ok(None),
           };
}

pub fn register(email: &str, name: &str, pwd: &str) -> LoginResult<UserAccount> {
    use self::schema::user_account;

    let encrypted_password = encrypt(pwd);
    let new_user = NewUserAccount {
        email: email.to_owned(),
        name: name.to_owned(),
        password: encrypted_password,
    };

    let conn = establish_connection();
    diesel::insert(&new_user).into(user_account::table).get_result(&conn)
}
