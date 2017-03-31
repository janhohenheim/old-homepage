#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate iron;
extern crate homepage;

use iron::Iron;
use homepage::*;

fn main() {
    let mut chain = routing::create_chain();
    templating::link_to_chain(&mut chain).unwrap();
    session::link_to_chain(&mut chain).unwrap();
    
    Iron::new(chain).http("localhost:8080").unwrap();
}
