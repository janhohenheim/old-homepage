extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use self::iron::{Request, Response, status, IronResult};
use self::iron::prelude::*;
use self::mount::Mount;
use self::staticfile::Static;
use std::path::Path;
use templating::{generate_site, generate_site_without_data, Section};
use quiz::controller::*;

pub fn create_chain() -> Chain {

    let router =
        router!(root: get "/" => handle_root,
                         contact: get "/contact" => handle_contact,
                         quiz: get "/quiz" => handle_quiz,
                         quiz_post: post "/quiz" => handle_quiz_post,
                         quiz_play: get "/quiz/play" => handle_quiz_play,
                         quiz_play_post: post "/quiz/play" => handle_quiz_play_post,
                         quiz_admin: get "/quiz/admin" => handle_quiz_admin,
                         quiz_admin_post: post "/quiz/admin" => handle_quiz_admin_post,);

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount
        .mount("/css", Static::new(Path::new("res/public/css")))
        .mount("/js", Static::new(Path::new("res/public/js")));

    Chain::new(mount)
}

fn respond_with_file(filename: &str, section: Option<&Section>) -> IronResult<Response> {
    let site_template = generate_site_without_data(filename, section);
    Ok(Response::with((site_template, status::Ok)))
}

fn respond_with_quiz_file(filename: &str) -> IronResult<Response> {
    respond_with_file(filename, Some(&Section::Quiz))
}

fn handle_root(_: &mut Request) -> IronResult<Response> {
    respond_with_file("index", Some(&Section::Home))
}

fn handle_contact(_: &mut Request) -> IronResult<Response> {
    respond_with_file("contact/contact", Some(&Section::Contact))
}

fn handle_quiz(req: &mut Request) -> IronResult<Response> {
    let path = get_start(req)?;
    respond_with_quiz_file(&path)
}

fn handle_quiz_post(req: &mut Request) -> IronResult<Response> {
    let path = post_start(req)?;
    respond_with_quiz_file(&path)
}

fn handle_quiz_play(_: &mut Request) -> IronResult<Response> {
    respond_with_quiz_file("quiz/quiz_start")
}

fn handle_quiz_play_post(_: &mut Request) -> IronResult<Response> {
    respond_with_quiz_file("quiz/quiz_question")
}

fn handle_quiz_admin(req: &mut Request) -> IronResult<Response> {
    get_admin(req)
}

fn handle_quiz_admin_post(req: &mut Request) -> IronResult<Response> {
    let path = post_admin(req)?;
    respond_with_quiz_file(&path)
}
