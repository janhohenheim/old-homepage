extern crate iron;
extern crate router;
extern crate handlebars;

use self::iron::{Request, IronResult, Response, status};
use self::handlebars::to_json;
use presentation::helper::templating::*;
use presentation::model::section::Section;
use presentation::model::rank::Rank;
use presentation::helper::session;
use presentation::helper::util::{get_formdata, to_ironresult, redirect};
use business::crud::remove_round;
use business::round_data::*;

pub fn get_score(req: &mut Request) -> IronResult<Response> {

    let round_data = to_ironresult(get_all_round_data())?;
    let mut ranks = round_data
        .into_iter()
        .filter(|x| !x.is_last_answer_wrong)
        .map(|x| {
            let points = x.answer_count as i32 * 30;
            let mut categories: String = x.categories
                .into_iter()
                .map(move |y| y.text)
                .fold("".to_owned(), |mut acc, x| {
                    acc.push_str(&x);
                    acc.push_str(", ");
                    acc
                });
            categories.pop();
            categories.pop();
            let game_length = x.end_time
                .signed_duration_since(x.start_time)
                .num_seconds() as i32;
            let game_start = x.start_time.format("%Y-%m-%d %H:%M:%S").to_string();
            Rank {
                round_id: x.id,
                name: x.player.name,
                score: points / game_length,
                points: points,
                game_start: game_start,
                game_length: game_length,
                categories: categories,
            }
        })
        .filter(|x| x.score != 0)
        .collect::<Vec<Rank>>();
    ranks.sort_by(|a, b| b.score.cmp(&a.score));
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
    let id = get_formdata(req, "round_id")?;
    let id_as_int = to_ironresult(id.parse::<i32>())?;
    to_ironresult(remove_round(id_as_int))?;
    redirect(req, "get_quiz_score")
}
