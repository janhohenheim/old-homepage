extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use std;
use iron::{Iron, Request, Response, status, IronResult};
use iron::prelude::*;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use router::Router;
use self::mount::Mount;
use self::staticfile::Static;

use ::templating::{make_site, Section};

pub fn create_chain() -> Chain {

    let router = router!(root: get "/" => handle_root,
                         contact: get "/contact" => handle_contact,
                         quiz: get "/quiz" => handle_quiz);

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/res/public/", Static::new(Path::new("res/public/")));

    Chain::new(mount)
}

fn handle_root(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let content_type = mime!(Text / Html);
    resp.set_mut(make_site(Section::Home, "Ayyyy")).set_mut(status::Ok);
    Ok(resp)
}

fn handle_contact(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Ferner Fanclub")))
}

fn handle_quiz(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Quizbois")))
}


fn get_site(path: &str) -> String {
    match File::open(path) {
        Err(_) => return get_site_not_found(path),
        Ok(mut val) => {
            let mut site = String::new();
            match val.read_to_string(&mut site) {
                Err(err) => return get_site_err(err),
                Ok(_) => return site,
            }
        }
    }
}

fn get_site_not_found(path: &str) -> String {
    let msg = format!("404, did not find site at {}", path);
    println!("{}", msg);
    msg
}

fn get_site_err<T: std::fmt::Display>(err: T) -> String {
    let msg = format!("Server error happened\n{}", err);
    println!("{}", msg);
    msg
}