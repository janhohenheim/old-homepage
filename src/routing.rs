extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use iron::{Request, Response, status, IronResult};
use iron::prelude::*;
use self::mount::Mount;
use self::staticfile::Static;
use std::path::Path;
use templating::{make_site_from_file, Section};

pub fn create_chain() -> Chain {

    let router = router!(root: get "/" => handle_root,
                         contact: get "/contact" => handle_contact,
                         quiz: get "/quiz" => handle_quiz);

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/css", Static::new(Path::new("res/public/css")))
        .mount("/js", Static::new(Path::new("res/public/js")));

    Chain::new(mount)
}

fn handle_root(_: &mut Request) -> IronResult<Response> {
    respond_with_file(&Section::Home, "index.html")
}

fn handle_contact(_: &mut Request) -> IronResult<Response> {
    respond_with_file(&Section::Contact, "contact/contact.html")
}

fn handle_quiz(_: &mut Request) -> IronResult<Response> {
    respond_with_file(&Section::Quiz, "quiz/quiz.hbs")
}

fn respond_with_file(section: &Section, filename: &str) -> IronResult<Response> {
    let site_template = make_site_from_file(section, filename);
    Ok(Response::with((site_template, status::Ok)))
}
