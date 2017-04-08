extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;
extern crate handlebars_iron as hbs;
extern crate serde_json;

use self::iron::{Request, IronResult, Response, status};
use self::handlebars::to_json;
use self::serde_json::Value;

use presentation::helper::util::{get_formdata, to_ironresult, redirect};
use presentation::helper::templating::*;
use presentation::helper::session;
use presentation::model::category::Category;
use presentation::model::section::Section;
use business::crud::*;
use std::collections::BTreeMap;

pub fn get_quiz(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_some() {
        return redirect(req, "get_quiz_play");
    }
    generate_quiz(req)
}

pub fn post_quiz(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_some() || create_player_data(req).is_ok() {
        return redirect(req, "get_quiz_play");
    }
    let err = btreemap!{
        "error".to_string() => to_json(&"true".to_string())
    };

    generate_quiz_start_with_data(req, err)
}


fn create_player_data(req: &mut Request) -> IronResult<()> {
    let name = get_formdata(req, "name")?;
    let category_ids = get_formdata(req, "categories")?;
    println!("{}", category_ids);
    let categories = Vec::new();
    let new_player = create_player(&name);
    let new_player = to_ironresult(new_player)?;

    session::create_player(req, new_player.id, categories)
}


fn generate_quiz(req: &mut Request) -> IronResult<Response> {
    generate_quiz_start_with_data(req, BTreeMap::new())
}

fn generate_quiz_start_with_data(req: &mut Request,
                                 mut data: BTreeMap<String, Value>)
                                 -> IronResult<Response> {
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
    let mut json = btreemap! {
        "categories".to_string() => to_json(&categories),
    };
    json.append(&mut data);
    let template = generate_site(req, "quiz/quiz_start", json, Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}
