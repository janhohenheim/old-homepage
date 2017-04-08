extern crate iron;
extern crate router;
extern crate handlebars;

use self::iron::{Request, IronResult, Response, status};
use self::iron::modifiers::Redirect;
use self::handlebars::to_json;
use presentation::helper::templating::*;
use presentation::model::section::Section;
use presentation::model::rank::Rank;
use presentation::helper::session;
use presentation::helper::util::{get_formdata, to_ironresult, redirect};
use business::crud::remove_round;

pub fn get_score(req: &mut Request) -> IronResult<Response> {
    let dummy = Rank {
        ranking: 1,
        name: "Foo".to_owned(),
        score: 20,
        points: 40,
        game_start: "14:00".to_owned(),
        game_length: 2,
        categories: "Memes, Bourgeoisie".to_owned(),
    };

    let ranks = vec![dummy];

    let data = btreemap! {
        "ranks".to_string() => to_json(&ranks),
    };

    let template = generate_site(req, "quiz/scoreboard", data, Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}


pub fn post_score_remove(req: &mut Request) -> IronResult<Response> {
    if session::get_admin(req)?.is_none() {
        return redirect(req, "get_root");
    }

    let id = get_formdata(req, "id")?;
    let id_as_int = to_ironresult(id.parse::<i32>())?;
    to_ironresult(remove_round(id_as_int))?;
    Ok(Response::with((status::Found, Redirect(url_for!(req, "get_quiz_score")))))
}
