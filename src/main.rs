extern crate iron;
#[macro_use]
extern crate mime;
#[macro_use]
extern crate router;
extern crate hyper_native_tls;

use iron::{Iron, Request, Response, status, IronResult};
use router::{Router};

fn main() {
    let router = router!(root: get "/" => handle_root,
                         query: get "/:query" => handle_query);
    Iron::new(router).http("127.0.0.1:8080").unwrap();
}

fn handle_root(_: &mut Request) -> IronResult<Response> {
    let content_type = mime!(Text / Html);
    Ok(Response::with((content_type,
                       status::Ok,
                       "<h1>Hello world!</h1><br /><h2>meeemes</h2><br/>:)")))
}

fn handle_query(req: &mut Request) -> IronResult<Response> {
    let query = req.extensions.get::<Router>()
        .unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, query)))
}
