#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate iron;
#[macro_use]
extern crate router;
extern crate handlebars;
extern crate handlebars_iron as hbs;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod templating;
pub mod routing;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
