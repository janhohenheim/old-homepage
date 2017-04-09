extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;
extern crate handlebars_iron as hbs;
extern crate serde_json;

use self::iron::{Request, IronResult, Response, status, IronError};
use self::handlebars::to_json;
use self::serde_json::Value;

use presentation::helper::util::{get_formdata, to_ironresult, redirect};
use presentation::helper::templating::*;
use presentation::helper::session;
use presentation::model::category::Category;
use presentation::model::section::Section;
use business::quiz::{start_game, is_game_in_progress};
use business::crud::*;
use std::collections::BTreeMap;
use std::error;
use std::fmt;

pub fn get_quiz(req: &mut Request) -> IronResult<Response> {
    if let Some(player) = session::get_player(req)? {
        if !to_ironresult(is_game_in_progress(player.id))? {
            session::clear(req)?;
        } else {
            return redirect(req, "get_quiz_play");
        }
    }
    generate_quiz(req)
}

pub fn post_quiz(req: &mut Request) -> IronResult<Response> {
    if session::get_player(req)?.is_some() {
        return redirect(req, "get_quiz_play");
    }
    if create_player_data(req).is_ok() {
        let player = session::get_player(req)?.unwrap();
        to_ironresult(start_game(player.id,
                                 player
                                     .categories
                                     .iter()
                                     .map(|x| x.id)
                                     .collect::<Vec<i32>>()))?;
        return redirect(req, "get_quiz_play");
    }
    let mut err = btreemap!{
        "error".to_string() => to_json(&"true".to_string())
    };
    if let Ok(name) = get_formdata(req, "name") {
        err.insert("name".to_string(), to_json(&name));
    }

    generate_quiz_start_with_data(req, err)
}


fn create_player_data(req: &mut Request) -> IronResult<()> {
    //Workaround because of limitations in iron-urlencoded
    let mut category_ids = Vec::new();
    let form_name = "playing_categories".to_owned();
    let category_count = to_ironresult(get_categories())?.len();
    for i in 0..category_count {
        let mut this_form = form_name.clone();
        this_form.push_str(&i.to_string());
        if let Ok(id) = get_formdata(req, &this_form) {
            let id_as_int = to_ironresult(id.parse::<i32>())?;
            category_ids.push(id_as_int);
        }
    }
    if category_ids.is_empty() {
        return Err(IronError {
                       error: Box::new(CategoryError {}),
                       response: Response::with(status::BadRequest),
                   });
    }

    let categories = category_ids
        .into_iter()
        .map(|x| {
                 let cat = get_category(x).unwrap();
                 Category {
                     id: cat.id,
                     text: cat.text,
                 }
             })
        .collect::<Vec<Category>>();

    let name = get_formdata(req, "name")?;
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



#[derive(Debug)]
pub struct CategoryError;

impl fmt::Display for CategoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Category error: No category has been selected")
    }
}

impl error::Error for CategoryError {
    fn description(&self) -> &str {
        "Category error"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
