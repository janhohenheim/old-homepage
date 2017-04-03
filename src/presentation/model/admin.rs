extern crate iron;
extern crate iron_sessionstorage;
extern crate serde;
extern crate serde_json;

use self::serde_json::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Admin {
    pub id: i32,
    pub name: String,
}

impl Admin {
    pub fn new(id: i32, name: String) -> Self {
        Admin { id, name }
    }
}

//TODO: exchange for custom [#derive], maybe do a pr on iron_sessionstorage
impl iron_sessionstorage::Value for Admin {
    fn get_key() -> &'static str {
        "admin"
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
