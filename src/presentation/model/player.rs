extern crate iron;
extern crate iron_sessionstorage;
extern crate serde;
extern crate serde_json;

use self::serde_json::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Player {
    pub id: i32,
    pub selected_answer_index: Option<i32>,
}

impl Player {
    pub fn new(id: i32) -> Self {
        Player {
            id,
            selected_answer_index: None,
        }
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
