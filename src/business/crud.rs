extern crate diesel;
extern crate chrono;

use self::chrono::offset::utc::UTC;
use self::diesel::prelude::*;
use data::model::quiz::category::*;
use data::model::quiz::player::*;
use data::model::quiz::question::*;
use data::model::quiz::answer::*;
use data::model::quiz::round::*;
use data::model::quiz::round_category::*;
use data::model::quiz::round_question::*;
use data::establish_connection;
use data::schema;
use self::diesel::result::{Error, DatabaseErrorKind};

type Result<T> = self::diesel::QueryResult<T>;

pub fn create_player(name: &str) -> Result<Player> {
    if name.is_empty() || name.len() > 15 {
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Name cannot be empty or over 15 chars"
                                                     .to_owned())));
    }
    use self::schema::player;
    let new_player = NewPlayer { name };
    let conn = establish_connection();
    diesel::insert(&new_player)
        .into(player::table)
        .get_result(&conn)
}

pub fn get_player(player_id: i32) -> Result<Player> {
    use self::schema::player::dsl::*;
    let conn = establish_connection();
    Ok(player.find(player_id).first(&conn)?)
}

pub fn create_category(cat_text: &str) -> Result<Category> {
    if cat_text.is_empty() || cat_text.len() > 140 {
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Text cannot be empty or too long".to_owned())));
    }
    use self::schema::category;
    use self::schema::category::dsl::*;
    let conn = establish_connection();
    let already_created_cats = category.filter(text.eq(cat_text))
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
    Ok(category.find(cat_id)
           .filter(is_active.eq(true))
           .first(&conn)?)
}

pub fn get_categories() -> Result<Vec<Category>> {
    use self::schema::category::dsl::*;
    let conn = establish_connection();
    category
        .filter(is_active.eq(true))
        .order(id.desc())
        .load::<Category>(&conn)
}

pub fn rename_category(cat_id: i32, cat_text: &str) -> Result<Category> {
    if cat_text.is_empty() || cat_text.len() > 140 {
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
    if q_text.is_empty() || q_text.len() > 140 {
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Text cannot be empty or too long".to_owned())));
    }
    use self::schema::question;
    use self::schema::question::dsl::*;
    let conn = establish_connection();

    let already_created_qs = question.filter(text.eq(q_text))
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
        .order(id.desc())
        .load::<Question>(&conn)
}

pub fn get_questions_with_category(cat_id: i32) -> Result<Vec<Question>> {
    use self::schema::question::dsl::*;
    let conn = establish_connection();
    question
        .filter(is_active.eq(true))
        .filter(category_id.eq(cat_id))
        .order(id.desc())
        .load::<Question>(&conn)
}

pub fn change_question_category(q_id: i32, cat_id: i32) -> Result<Question> {
    let conn = establish_connection();
    use self::schema::question::dsl::*;
    diesel::update(question.find(q_id))
        .set(category_id.eq(cat_id))
        .get_result::<Question>(&conn)
}

pub fn rename_question(q_id: i32, q_text: &str) -> Result<Question> {
    if q_text.is_empty() || q_text.len() > 140 {
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
    if a_text.is_empty() || a_text.len() > 140 {
        return Err(Error::DatabaseError(DatabaseErrorKind::__Unknown,
                                        Box::new("Text cannot be empty or too long".to_owned())));
    }
    use self::schema::answer;
    use self::schema::answer::dsl::*;
    let already_created_as = answer.filter(text.eq(a_text))
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

pub fn get_answer(a_id: i32) -> Result<Answer> {
    use self::schema::answer::dsl::*;
    let conn = establish_connection();
    answer.find(a_id).first::<Answer>(&conn)
}

pub fn get_answers(q_id: i32) -> Result<Vec<Answer>> {
    use self::schema::answer::dsl::*;
    let conn = establish_connection();
    answer
        .filter(question_id.eq(q_id))
        .filter(is_active.eq(true))
        .order(id.desc())
        .load::<Answer>(&conn)
}

pub fn rename_answer(a_id: i32, a_text: &str) -> Result<Answer> {
    if a_text.is_empty() || a_text.len() > 140 {
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

pub fn get_player_rounds(p_id: i32) -> Result<Vec<Round>> {
    use self::schema::round::dsl::*;
    let conn = establish_connection();
    round
        .filter(player_id.eq(p_id))
        .order(id.desc())
        .load::<Round>(&conn)
}

pub fn create_round(p_id: i32) -> Result<Round> {
    use self::schema::round;
    let conn = establish_connection();

    let new_round = NewRound { player_id: p_id };
    diesel::insert(&new_round)
        .into(round::table)
        .get_result(&conn)
}

pub fn get_round(round_id: i32) -> Result<Round> {
    use self::schema::round::dsl::*;
    let conn = establish_connection();
    round.find(round_id).first::<Round>(&conn)
}


pub fn get_rounds() -> Result<Vec<Round>> {
    use self::schema::round::dsl::*;
    let conn = establish_connection();
    round.order(id.desc()).load::<Round>(&conn)
}

pub fn set_round_finished(round_id: i32) -> Result<Round> {
    use self::schema::round::dsl::*;
    let conn = establish_connection();
    diesel::update(round.find(round_id))
        .set(is_finished.eq(true))
        .get_result::<Round>(&conn)
}

pub fn remove_round(round_id: i32) -> Result<bool> {
    use self::schema::round::dsl::*;
    let conn = establish_connection();
    let num_deleted = diesel::delete(round.find(round_id)).execute(&conn)?;
    Ok(num_deleted != 0)
}

pub fn create_round_category(r_id: i32, cat_id: i32) -> Result<RoundCategory> {
    use self::schema::round_category;
    let conn = establish_connection();

    let new_round_category = NewRoundCategory {
        round_id: r_id,
        category_id: cat_id,
    };
    let a = diesel::insert(&new_round_category)
        .into(round_category::table)
        .get_result(&conn);
    a
}

pub fn get_round_categories(r_id: i32) -> Result<Vec<RoundCategory>> {
    use self::schema::round_category::dsl::*;
    let conn = establish_connection();
    round_category
        .filter(round_id.eq(r_id))
        .order(id.desc())
        .load::<RoundCategory>(&conn)
}

pub fn get_round_categories_joined(r_id: i32) -> Result<Vec<Category>> {
    use self::schema::round_category;
    use self::schema::round_category::dsl::*;
    use self::schema::category;
    let conn = establish_connection();
    let data: Vec<(Category, RoundCategory)> = category::table.inner_join(round_category::table)
        .filter(round_id.eq(r_id))
        .order(id.desc())
        .load(&conn)?;
    let (cats, _): (Vec<Category>, Vec<RoundCategory>) = data.into_iter().unzip();
    Ok(cats)
}

pub fn create_round_question(r_id: i32, q_id: i32) -> Result<RoundQuestion> {
    use self::schema::round_question;
    let conn = establish_connection();

    let new_round_question = NewRoundQuestion {
        round_id: r_id,
        question_id: q_id,
    };

    diesel::insert(&new_round_question)
        .into(round_question::table)
        .get_result(&conn)
}

pub fn get_round_questions(r_id: i32) -> Result<Vec<RoundQuestion>> {
    use self::schema::round_question::dsl::*;
    let conn = establish_connection();
    round_question
        .filter(round_id.eq(r_id))
        .order(id.desc())
        .load::<RoundQuestion>(&conn)
}

pub fn get_round_questions_joined(r_id: i32) -> Result<Vec<Question>> {
    use self::schema::round_question;
    use self::schema::round_question::dsl::*;
    use self::schema::question;
    let conn = establish_connection();
    let data: Vec<(Question, RoundQuestion)> = question::table.inner_join(round_question::table)
        .filter(round_id.eq(r_id))
        .order(id.desc())
        .load(&conn)?;
    let (qs, _): (Vec<Question>, Vec<RoundQuestion>) = data.into_iter().unzip();
    Ok(qs)
}

pub fn set_joker_user(round_question_id: i32) -> Result<RoundQuestion> {
    use self::schema::round_question::dsl::*;
    let conn = establish_connection();
    diesel::update(round_question.find(round_question_id))
        .set(is_joker_used.eq(true))
        .get_result::<RoundQuestion>(&conn)
}

pub fn set_end_time_to_now(round_question_id: i32) -> Result<RoundQuestion> {
    use self::schema::round_question::dsl::*;
    let conn = establish_connection();
    diesel::update(round_question.find(round_question_id))
        .set(end_time.eq(UTC::now().naive_utc()))
        .get_result::<RoundQuestion>(&conn)

}

pub fn set_answer(round_question_id: i32, a_id: i32) -> Result<RoundQuestion> {
    use self::schema::round_question::dsl::*;
    let conn = establish_connection();
    diesel::update(round_question.find(round_question_id))
        .set(answer_id.eq(a_id))
        .get_result::<RoundQuestion>(&conn)

}
