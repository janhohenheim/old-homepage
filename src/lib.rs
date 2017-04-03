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
#[macro_use]
extern crate mime;


pub mod routing;
pub mod presentation;
pub mod data;
pub mod business;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
