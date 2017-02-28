extern crate iron;
extern crate hyper_native_tls;

use iron::mime::{Mime, TopLevel, SubLevel};
use iron::headers::ContentType;

fn main() {
    // Avoid unused errors due to conditional compilation ('native-tls-example' feature is not default)
    use hyper_native_tls::NativeTlsServer;
    use iron::{Iron, Request, Response};
    use iron::status;
    use std::result::Result;

    let ssl = NativeTlsServer::new("identity.p12", "mypass").unwrap();

    match Iron::new(|_: &mut Request| {
        println!("Got an incoming connection!");
        let mut response = Response::with((status::Ok, "<h1>Hello world!</h1>"));
        response.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
        Ok(response)
    }).https("127.0.0.1:3000", ssl) {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
    // curl -vvvv https://127.0.0.1:3000/ -k
}