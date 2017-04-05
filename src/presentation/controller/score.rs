extern crate iron;
extern crate router;
extern crate handlebars;

use self::iron::{Request, IronResult, Response, status};
use self::handlebars::to_json;
use presentation::helper::templating::*;
use presentation::model::section::Section;
use presentation::model::rank::Rank;

pub fn get_score(req: &mut Request) -> IronResult<Response> {
    let dummy = Rank{
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
