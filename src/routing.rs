extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use self::iron::{Request, Response, status, IronResult};
use self::iron::prelude::*;
use self::mount::Mount;
use self::staticfile::Static;
use std::path::Path;
use presentation::helper::templating::generate_site_without_data;
use presentation::controller as ctrl;
use presentation::model::section::Section;

pub fn create_chain() -> Chain {
    use self::ctrl::*;
    let router =
        router!(get_root: get "/" => handle_root,
                         get_contact: get "/contact" => handle_contact,
                         post_login: post "/login" => login::post_login,
                         post_register: post "/register" => login::post_register,
                         get_logout: get "/logout" => login::get_logout,
                         get_quiz: get "/quiz" => quiz::get_quiz,
                         post_quiz: post "/quiz" => quiz::post_quiz,
                         get_quiz_play: get "/quiz/play" => play::get_play,
                         post_quiz_play: post "/quiz/play" => play::post_play,
                         get_quiz_score: get "/quiz/score" => score::get_score,
                         get_quiz_admin: get "/quiz/admin" => admin::get_admin,
                         post_quiz_admin_post: post "/quiz/admin" => admin::post_admin,);

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/css", Static::new(Path::new("public/css")));

    Chain::new(mount)
}

fn respond_with_file(req: &mut Request,
                     filename: &str,
                     section: Option<&Section>)
                     -> IronResult<Response> {
    let site_template = generate_site_without_data(req, filename, section);
    Ok(Response::with((site_template, status::Ok)))
}

fn handle_root(req: &mut Request) -> IronResult<Response> {
    respond_with_file(req, "index", Some(&Section::Home))
}

fn handle_contact(req: &mut Request) -> IronResult<Response> {
    respond_with_file(req, "contact/contact", Some(&Section::Contact))
}
