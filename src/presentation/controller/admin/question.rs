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
use presentation::model::answer::Answer;
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
                answers: get_presentation_answers(x.id).unwrap(),
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

fn get_presentation_answers(question_id: i32) -> IronResult<Vec<Answer>> {
    let data_answers = to_ironresult(get_answers(question_id))?;
    let converted_answers = data_answers
    .into_iter()
    .map(|x| {
        Answer{
            id: x.id,
            text: x.text,
            is_confirmed_wrong: !x.is_correct,
        }
    })
    .collect::<Vec<Answer>>();
    Ok(converted_answers)
}

pub fn post_question_add(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_none() {
        return redirect(req, "get_root");
    }
    if !validate_new_answers(req, 4)?{
        return Ok(Response::with((status::Found, Redirect(url_for!(req, "get_quiz_admin_question")))));
    }

    let category = get_formdata(req, "category_to_add")?;
    let category_as_int = to_ironresult(category.parse::<i32>())?;
    let question = get_formdata(req, "new_question")?;
    let new_question = create_question(category_as_int, &question);
    let new_question_id = to_ironresult(new_question)?.id;
    create_answer_from_form(req, 0, new_question_id)?;
    create_answer_from_form(req, 1, new_question_id)?;
    create_answer_from_form(req, 2, new_question_id)?;
    create_answer_from_form(req, 3, new_question_id)?;

    Ok(Response::with((status::Found, Redirect(url_for!(req, "get_quiz_admin_question")))))
}

fn validate_new_answers(req: &mut Request, question_count: i32) -> IronResult<bool> {
    validate_answers_on_forms(req, question_count, "new_correct_answer", "new_answer")
}

fn validate_edited_answers(req: &mut Request, question_count: i32) -> IronResult<bool> {
    validate_answers_on_forms(req, question_count, "correct_answer", "answer")
}

fn validate_answers_on_forms(req: &mut Request,
                             question_count: i32,
                             correct_answer: &str,
                             answer: &str)
                            -> IronResult<bool> {
    let correct_index = get_formdata(req, correct_answer)?;
    let correct_nr = to_ironresult(correct_index.parse::<i32>())?;
    if correct_nr >= question_count {
        return Ok(false);
    }
    let mut answers = Vec::new();
    let form_name = answer.to_owned();
    for i in 0..question_count {
        let mut this_name = form_name.clone();
        this_name.push_str(&i.to_string());
        answers.push(get_formdata(req, &this_name)?);
    }
    let orig_len = answers.len();
    answers.sort();
    answers.dedup();
    return Ok(answers.len() == orig_len)
}

fn create_answer_from_form(req: &mut Request, nr: i32, question_id: i32) -> IronResult<()> {
    let mut form_name = "new_answer".to_owned();
    form_name.push_str(&nr.to_string());
    let ans_text = &get_formdata(req, &form_name)?;
    let correct_answer = get_formdata(req, "new_correct_answer")?;
    let correct_nr = to_ironresult(correct_answer.parse::<i32>())?;
    let ans = create_answer(question_id, &ans_text, correct_nr == nr);
    to_ironresult(ans)?;
    Ok(())
}


pub fn post_question_edit(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_none() {
        return redirect(req, "get_root");
    }
    if !validate_edited_answers(req, 4)?{
        return Ok(Response::with((status::Found, Redirect(url_for!(req, "get_quiz_admin_question")))));
    }

    let question = get_formdata(req, "question_id")?;
    let question_id = to_ironresult(question.parse::<i32>())?;
    let text = get_formdata(req, "question_text")?;
    let edited_question = rename_question(question_id, &text);
    to_ironresult(edited_question)?;

    let cat = get_formdata(req, "category")?;
    let cat_id = to_ironresult(cat.parse::<i32>())?;
    let edited_question = change_question_category(question_id, cat_id);
    to_ironresult(edited_question)?;

    edit_answer_from_form(req, 0)?;
    edit_answer_from_form(req, 1)?;
    edit_answer_from_form(req, 2)?;
    edit_answer_from_form(req, 3)?;

    Ok(Response::with((status::Found, Redirect(url_for!(req, "get_quiz_admin_question")))))
}

fn edit_answer_from_form(req: &mut Request, nr: i32) -> IronResult<()> {
    let mut form_name = "answer".to_owned();
    let mut form_id_name = "answer_id".to_owned();
    form_name.push_str(&nr.to_string());
    form_id_name.push_str(&nr.to_string());

    let answer = &get_formdata(req, &form_id_name)?;
    let answer_id = to_ironresult(answer.parse::<i32>())?;
    let ans_text = &get_formdata(req, &form_name)?;

    let correct_answer = get_formdata(req, "correct_answer")?;
    let correct_nr = to_ironresult(correct_answer.parse::<i32>())?;

    let ans = rename_answer(answer_id, &ans_text);
    to_ironresult(ans)?;

    let ans = change_answer_correct(answer_id, correct_nr == nr);
    to_ironresult(ans)?;
    Ok(())
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
