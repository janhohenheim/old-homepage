use data::schema::user_account;

#[derive(Queryable, Identifiable)]
#[table_name="user_account"]
pub struct UserAccount {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name="user_account"]
pub struct NewUserAccount {
    pub email: String,
    pub name: String,
    pub password: String,
}
