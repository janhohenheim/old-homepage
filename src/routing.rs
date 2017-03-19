extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use iron::{Request, Response, status, IronResult};
use iron::prelude::*;
use self::mount::Mount;
use self::staticfile::Static;
use std::path::Path;
use templating::{make_site, make_site_from_file, Section};

pub fn create_chain() -> Chain {

    let router = router!(root: get "/" => handle_root,
                         contact: get "/contact" => handle_contact,
                         quiz: get "/quiz" => handle_quiz);

    let mut mount = Mount::new();
    mount.mount("/", router).mount("/res/public/", Static::new(Path::new("res/public/")));

    Chain::new(mount)
}

fn handle_root(_: &mut Request) -> IronResult<Response> {
    respond_with_file(Section::Home, "index.html")
}

fn handle_contact(_: &mut Request) -> IronResult<Response> {
    respond_with_text(Section::Contact, "Ferner Fanclub")
}

fn handle_quiz(_: &mut Request) -> IronResult<Response> {
    respond_with_text(Section::Quiz, "Quizbois")
}

fn respond_with_text(section: Section, content: &str) -> IronResult<Response> {
    let site_template = make_site(section, content);
    Ok(Response::with((site_template,
                       status::Ok)))
}

fn respond_with_file(section: Section, filename: &str) -> IronResult<Response> {
    let site_template = make_site_from_file(section, filename);
    Ok(Response::with((site_template,
                       status::Ok)))
}