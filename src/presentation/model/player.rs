extern crate iron;
extern crate iron_sessionstorage;
extern crate serde;
extern crate serde_json;

use self::serde_json::{from_str, to_string};
use super::category::Category;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Player {
    pub id: i32,
    pub categories: Vec<Category>,
}

impl Player {
    pub fn new(id: i32, categories: Vec<Category>) -> Self {
        Player { id, categories }
    }
}


impl iron_sessionstorage::Value for Player {
    fn get_key() -> &'static str {
        "player"
    }
    fn into_raw(self) -> String {
        to_string(&self).unwrap()
    }
    fn from_raw(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            from_str(&value).ok()
        }
    }
}
