extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;
extern crate handlebars_iron as hbs;

use self::iron::{Request, IronResult, Response, status};
use presentation::helper::templating::*;
use business::crud::{get_categories, create_category, rename_category, deactivate_category};
use self::iron::modifiers::Redirect;
use presentation::helper::session;
use presentation::model::category::Category;
use self::handlebars::to_json;
use presentation::helper::util::{get_formdata, to_ironresult, redirect};

pub fn get_category(req: &mut Request) -> IronResult<Response> {
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
    let cat_json = btreemap! {
        "categories".to_string() => to_json(&categories),
    };
    let template = generate_site(req, "quiz/admin/category", cat_json, None);
    Ok(Response::with((template, status::Ok)))
}

pub fn post_category_add(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_none() {
        return redirect(req, "get_root");
    }
    let category = get_formdata(req, "new_category")?;
    let new_category = create_category(&category);
    to_ironresult(new_category)?;
    Ok(Response::with((status::Found, Redirect(url_for!(req, "get_quiz_admin_cat")))))
}


pub fn post_category_edit(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_none() {
        return redirect(req, "get_root");
    }
    let id = get_formdata(req, "id")?;
    let id_as_int = to_ironresult(id.parse::<i32>())?;
    let text = get_formdata(req, "text")?;
    let edited_category = rename_category(id_as_int, &text);
    to_ironresult(edited_category)?;
    Ok(Response::with((status::Found, Redirect(url_for!(req, "get_quiz_admin_cat")))))
}


pub fn post_category_remove(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_none() {
        return redirect(req, "get_root");
    }
    let id = get_formdata(req, "id")?;
    let id_as_int = to_ironresult(id.parse::<i32>())?;
    let deactivated_category = deactivate_category(id_as_int);
    to_ironresult(deactivated_category)?;
    Ok(Response::with((status::Found, Redirect(url_for!(req, "get_quiz_admin_cat")))))
}
