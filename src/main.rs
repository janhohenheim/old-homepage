extern crate iron;
#[macro_use]
extern crate mime;
#[macro_use]
extern crate router;

use iron::{Iron, Request, Response, status, IronResult};
use router::Router;
use std::io::prelude::*;
use std::fs::File;


fn main() {
    let router = router!(root: get "/" => handle_root,
                         query: get "/:query" => handle_query,
                         quiz: get "/quiz" => handle_quiz);
    Iron::new(router).http("127.0.0.1:8080").unwrap();
}

fn handle_root(_: &mut Request) -> IronResult<Response> {
    let content_type = mime!(Text / Html);
    let site = get_site("res/index.html");
    Ok(Response::with((content_type, status::Ok, site)))
}

fn handle_query(req: &mut Request) -> IronResult<Response> {
    let query = req.extensions
        .get::<Router>()
        .unwrap()
        .find("query")
        .unwrap_or("/");
    Ok(Response::with((status::Ok, query)))
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
