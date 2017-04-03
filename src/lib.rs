#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate iron;
#[macro_use]
extern crate router;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate maplit;


pub mod routing;
pub mod templating;
pub mod session;
pub mod quiz;
pub mod util;
pub mod login_controller;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
