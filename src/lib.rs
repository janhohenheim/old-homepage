#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate router;
#[macro_use]
extern crate serde_derive;
pub mod routing;
pub mod templating;
pub mod session;
pub mod quiz_controller;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
