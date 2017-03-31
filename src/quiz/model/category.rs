use super::super::schema::{category, question};

#[derive(Queryable, Identifiable, Associations)]
#[has_many(question)]
#[table_name="category"]
pub struct Category {
    pub id: i32,
    pub text: String,
    pub is_active: bool,
}

#[derive(Insertable)]
#[table_name="category"]
pub struct NewCategory<'a> {
    pub text: &'a str,
}
