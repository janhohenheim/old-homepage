use data::schema::{category, question, round_category};

#[derive(Queryable, Identifiable, Associations)]
#[has_many(question)]
#[has_many(round_category)]
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
