extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;
extern crate handlebars_iron as hbs;

use self::iron::{Request, IronResult, Response, status};
use presentation::helper::templating::*;
use business::crud::*;
use self::iron::modifiers::Redirect;
use presentation::helper::session;
use presentation::model::category::Category;
use presentation::model::question::Question;
use self::handlebars::to_json;
use presentation::helper::util::{get_formdata, to_ironresult, redirect};

pub fn get_question(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_none() {
        return redirect(req, "get_root");
    }
    let categories = get_categories()
        .unwrap()
        .into_iter()
        .map(|x| {
                 Category {
                     id: x.id,
                     text: x.text,
                 }
             })
        .collect::<Vec<Category>>();
    let questions = get_questions()
        .unwrap()
        .into_iter()
        .map(|x| {
            let category = get_category(x.category_id).unwrap();
            Question {
                id: x.id,
                text: x.text,
                category: Category {
                    id: category.id,
                    text: category.text,
                },
                categories: &categories,
                answers: vec![],
            }
        })
        .collect::<Vec<Question>>();
    let json = btreemap! {
        "questions".to_string() => to_json(&questions),
        "categories".to_string() => to_json(&categories),
    };
    let template = generate_site(req, "quiz/admin/question", json, None);
    Ok(Response::with((template, status::Ok)))
}

pub fn post_question_add(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_none() {
        return redirect(req, "get_root");
    }
    let category = get_formdata(req, "category_to_add")?;
    let category_as_int = to_ironresult(category.parse::<i32>())?;
    let question = get_formdata(req, "new_question")?;
    let new_question = create_question(category_as_int, &question);
    to_ironresult(new_question)?;
    Ok(Response::with((status::Found, Redirect(url_for!(req, "get_quiz_admin_question")))))
}


pub fn post_question_edit(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_none() {
        return redirect(req, "get_root");
    }
    let id = get_formdata(req, "question_id")?;
    let id_as_int = to_ironresult(id.parse::<i32>())?;
    let text = get_formdata(req, "question_text")?;
    let edited_question = rename_question(id_as_int, &text);
    to_ironresult(edited_question)?;
    Ok(Response::with((status::Found, Redirect(url_for!(req, "get_quiz_admin_question")))))
}


pub fn post_question_remove(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_none() {
        return redirect(req, "get_root");
    }
    let id = get_formdata(req, "question_id")?;
    let id_as_int = to_ironresult(id.parse::<i32>())?;
    let deactivated_category = deactivate_question(id_as_int);
    to_ironresult(deactivated_category)?;
    Ok(Response::with((status::Found, Redirect(url_for!(req, "get_quiz_admin_question")))))
}
