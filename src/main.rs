extern crate iron;
extern crate homepage;

use iron::Iron;
use homepage::*;

fn main() {
    let mut chain = routing::create_chain();
    if let Err(_) = templating::link_to_chain(&mut chain) {
        panic!();
    }
    Iron::new(chain).http("localhost:8080").unwrap();
}
