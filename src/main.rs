extern crate iron;
extern crate hyper_native_tls;

use iron::mime::{Mime, TopLevel, SubLevel};
use iron::headers::ContentType;
use iron::{Iron, Request, Response, method};
use iron::status;
use std::result::Result;

fn main() {
    match Iron::new(|request: &mut Request| {
        println!("Got an incoming connection!");
        let mut response = Response::with((status::Ok, "<h1>Hello world!</h1><br /><h2>meeemes</h2><br/>:)"));
        response.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
        Ok(response)
    }).http("127.0.0.1:8080") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
}
