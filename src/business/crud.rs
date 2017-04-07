extern crate diesel;

use self::diesel::prelude::*;
use data::model::quiz::category::*;
use data::model::quiz::player::*;
use data::model::quiz::question::*;
use data::model::quiz::answer::*;
use data::establish_connection;
use data::schema;
use self::diesel::result::{Error, DatabaseErrorKind};

type Result<T> = self::diesel::QueryResult<T>;

pub fn create_player(name: &str) -> Result<Player> {
    if name.is_empty() || name.len() > 15 {
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Name cannot be empty or over 15 chars".to_owned())));
    }
    use self::schema::player;
    let new_player = NewPlayer { name };
    let conn = establish_connection();
    diesel::insert(&new_player)
        .into(player::table)
        .get_result(&conn)
}

pub fn create_category(cat_text: &str) -> Result<Category> {
    if cat_text.is_empty() || cat_text.len() > 140{
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Text cannot be empty or too long".to_owned())));
    }
    use self::schema::category;
    use self::schema::category::dsl::*;
    let conn = establish_connection();
    let already_created_cats = category
            .filter(text.eq(cat_text))
            .filter(is_active.eq(true))
            .load::<Category>(&conn)?;
    if !already_created_cats.is_empty() {
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Text cannot be empty".to_owned())));
    }

    let new_category = NewCategory { text: cat_text };
    diesel::insert(&new_category)
        .into(category::table)
        .get_result(&conn)
}

pub fn get_category(cat_id: i32) -> Result<Category> {
    use self::schema::category::dsl::*;
    let conn = establish_connection();
    Ok(category
           .find(cat_id)
           .first(&conn)?)
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
    if cat_text.is_empty() || cat_text.len() > 140{
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Text cannot be empty or too long".to_owned())));
    }
    use self::schema::category::dsl::*;
    let conn = establish_connection();
    diesel::update(category.find(cat_id))
        .set(text.eq(cat_text))
        .get_result::<Category>(&conn)
}

pub fn deactivate_category(cat_id: i32) -> Result<Category> {
    use self::schema::category::dsl::*;
    let conn = establish_connection();
    diesel::update(category.find(cat_id))
        .set(is_active.eq(false))
        .get_result::<Category>(&conn)
}


pub fn create_question(q_category_id: i32, q_text: &str) -> Result<Question> {
    if q_text.is_empty() || q_text.len() > 140{
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Text cannot be empty or too long".to_owned())));
    }
    use self::schema::question;
    use self::schema::question::dsl::*;
    let conn = establish_connection();

    let already_created_qs = question
        .filter(text.eq(q_text))
        .filter(is_active.eq(true))
        .load::<Question>(&conn)?;
    if !already_created_qs.is_empty() {
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Already exists".to_owned())));
    }

    let new_question = NewQuestion {
        text: q_text,
        category_id: q_category_id,
    };
    diesel::insert(&new_question)
        .into(question::table)
        .get_result(&conn)
}

pub fn get_questions() -> Result<Vec<Question>> {
    use self::schema::question::dsl::*;
    let conn = establish_connection();
    question
        .filter(is_active.eq(true))
        .order(category_id.asc())
        .load::<Question>(&conn)
}

pub fn change_question_category(q_id: i32, cat_id: i32) -> Result<Question> {
    let conn = establish_connection();
    {
        use self::schema::category::dsl::*;
        category.find(cat_id).first::<Category>(&conn)?;
    }
    use self::schema::question::dsl::*;
    diesel::update(question.find(q_id))
        .set(category_id.eq(cat_id))
        .get_result::<Question>(&conn)
}

pub fn rename_question(q_id: i32, q_text: &str) -> Result<Question> {
    if q_text.is_empty() || q_text.len() > 140{
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Text cannot be empty or too long".to_owned())));
    }
    use self::schema::question::dsl::*;
    let conn = establish_connection();
    diesel::update(question.find(q_id))
        .set(text.eq(q_text))
        .get_result::<Question>(&conn)
}

pub fn deactivate_question(q_id: i32) -> Result<Question> {
    use self::schema::question::dsl::*;
    let conn = establish_connection();
    diesel::update(question.find(q_id))
        .set(is_active.eq(false))
        .get_result::<Question>(&conn)
}

pub fn create_answer(a_question_id: i32, a_text: &str, a_is_correct: bool) -> Result<Answer> {
    let conn = establish_connection();
    if a_text.is_empty() || a_text.len() > 140{
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Text cannot be empty or too long".to_owned())));
    }
    {
        use self::schema::question::dsl::*;
        question.find(a_question_id).first::<Question>(&conn)?;
    }
    use self::schema::answer;
    use self::schema::answer::dsl::*;
    let already_created_as = answer
        .filter(text.eq(a_text))
        .filter(question_id.eq(a_question_id))
        .filter(is_active.eq(true))
        .load::<Answer>(&conn)?;
    if !already_created_as.is_empty() {
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Already exists".to_owned())));
    }
    let new_answer = NewAnswer {
        text: a_text,
        is_correct: a_is_correct,
        question_id: a_question_id,
    };
    diesel::insert(&new_answer)
        .into(answer::table)
        .get_result(&conn)
}

pub fn get_answers(q_id: i32) -> Result<Vec<Answer>> {
    use self::schema::answer::dsl::*;
    let conn = establish_connection();
    answer
        .filter(question_id.eq(q_id))
        .order(id.asc())
        .load::<Answer>(&conn)
}

pub fn rename_answer(a_id: i32, a_text: &str) -> Result<Answer> {
    if a_text.is_empty() || a_text.len() > 140{
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Text cannot be empty or too long".to_owned())));
    }
    use self::schema::answer::dsl::*;
    let conn = establish_connection();
    diesel::update(answer.find(a_id))
        .set(text.eq(a_text))
        .get_result::<Answer>(&conn)
}

pub fn change_answer_correct(a_id: i32, state: bool) -> Result<Answer> {
    use self::schema::answer::dsl::*;
    let conn = establish_connection();
    diesel::update(answer.find(a_id))
        .set(is_correct.eq(state))
        .get_result::<Answer>(&conn)
}