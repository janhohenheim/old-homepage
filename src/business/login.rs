extern crate diesel;

use self::diesel::prelude::*;
use data::schema;
use data::model::user_account::*;
use data::establish_connection;
use super::crypto::{encrypt, decrypt};

type LoginResult<T> = self::diesel::QueryResult<T>;

pub fn login(user_email: &str, pwd: &str) -> LoginResult<Option<UserAccount>> {
    use self::schema::user_account::dsl::*;

    let decrypted_password = decrypt(pwd);
    let conn = establish_connection();
    let mut found_user = user_account.filter(email.eq(user_email))
        .filter(password.eq(&decrypted_password))
        .load::<UserAccount>(&conn)?;

    let result = match found_user.is_empty() {
        true => None,
        false => Some(found_user.remove(0)),
    };
    Ok(result)
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
