use data::schema::round_category;
use super::category::Category;
use super::round::Round;


#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Category)]
#[belongs_to(Round)]
#[table_name="round_category"]
pub struct RoundCategory {
    pub id: i32,
    pub round_id: i32,
    pub category_id: i32,
}

#[derive(Insertable)]
#[table_name="round_category"]
pub struct NewRoundCategory {
    pub round_id: i32,
    pub category_id: i32,
}
