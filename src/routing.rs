extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use self::iron::{Request, Response, status, IronResult};
use self::iron::prelude::*;
use self::mount::Mount;
use self::staticfile::Static;
use std::path::{Path};
use templating::{make_site_from_file, Section};
use quiz::controller::{start, start_post};

pub fn create_chain() -> Chain {

    let router = router!(root: get "/" => handle_root,
                         contact: get "/contact" => handle_contact,
                         quiz: get "/quiz" => handle_quiz,
                         quiz_post: post "/quiz" => handle_quiz_post,
                         quiz_play: get "/quiz/play" => handle_quiz_play,
                         quiz_play: post "/quiz/play" => handle_quiz_play_post);

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/css", Static::new(Path::new("res/public/css")))
        .mount("/js", Static::new(Path::new("res/public/js")));

    Chain::new(mount)
}

fn respond_with_file(section: Option<&Section>, filename: &Path) -> IronResult<Response> {
    let site_template = make_site_from_file(section, filename);
    Ok(Response::with((site_template, status::Ok)))
}

fn handle_root(_: &mut Request) -> IronResult<Response> {
    respond_with_file(Some(&Section::Home), Path::new("index.html"))
}

fn handle_contact(_: &mut Request) -> IronResult<Response> {
    respond_with_file(Some(&Section::Contact), Path::new("contact/contact.html"))
}

fn handle_quiz(req: &mut Request) -> IronResult<Response> {
    let path = start(req)?;
    respond_with_file(Some(&Section::Quiz), &path)
}

fn handle_quiz_post(req: &mut Request) -> IronResult<Response> {
    let path = start_post(req)?;
    respond_with_file(Some(&Section::Quiz), &path)
}

fn handle_quiz_play(_: &mut Request) -> IronResult<Response> {
    respond_with_file(Some(&Section::Quiz), Path::new("quiz/quiz_start.hbs"))
}

fn handle_quiz_play_post(_: &mut Request) -> IronResult<Response> {
    respond_with_file(Some(&Section::Quiz), Path::new("quiz/quiz_question.hbs"))
}
