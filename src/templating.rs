extern crate iron;
extern crate mime;
extern crate router;
extern crate handlebars;
extern crate handlebars_iron as hbs;
extern crate serde;
extern crate serde_json;

use std;
use self::iron::prelude::*;
use self::hbs::{Template, HandlebarsEngine, DirectorySource, SourceError};
use self::handlebars::to_json;
use self::serde_json::value::Map;
use std::fs::File;
use std::io::prelude::*;


#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum Section {
    Home,
    Quiz,
    Contact,
}

pub fn link_to_chain(chain: &mut Chain) -> Result<&mut Chain, SourceError> {
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./res/templates/", ".hbs")));
    hbse.reload()?;
    Ok(chain.link_after(hbse))
}


#[derive(Serialize, Debug)]
struct SectionData {
    name: Section,
    route: String,
    is_active: bool,
}


pub fn make_site(section: Section, content: &str) -> Template {
    let mut sections = get_sections();
    set_active_section(&mut sections, section);

    let mut data = Map::new();
    data.insert("sections".to_string(), to_json(&sections));
    data.insert("content".to_string(), to_json(&content.to_owned()));

    Template::new("frame", data)
}

pub fn make_site_from_file(section: Section, path: &str) -> Template {
    make_site(section, &get_site(&path))
}

fn get_sections() -> Vec<SectionData> {
    vec![SectionData {
             name: Section::Home,
             route: "/".to_string(),
             is_active: false,
         },
         SectionData {
             name: Section::Quiz,
             route: "/quiz".to_string(),
             is_active: false,
         },
         SectionData {
             name: Section::Contact,
             route: "/contact".to_string(),
             is_active: false,
         }]
}

fn set_active_section(sections: &mut Vec<SectionData>, active: Section) {
    for section in sections {
        if section.name == active {
            section.is_active = true
        }
    }
}

fn get_site(path: &str) -> String {
    let mut whole_path = "res/templates/".to_string();
    whole_path.push_str(path);
    match File::open(&whole_path) {
        Err(_) => return get_site_not_found(path),
        Ok(mut val) => {
            let mut site = String::new();
            match val.read_to_string(&mut site) {
                Err(err) => return get_site_err(err),
                Ok(_) => return site,
            }
        }
    }
}

fn get_site_not_found(path: &str) -> String {
    let msg = format!("404, did not find site at {}", path);
    println!("{}", msg);
    msg
}

fn get_site_err<T: std::fmt::Display>(err: T) -> String {
    let msg = format!("Server error happened\n{}", err);
    println!("{}", msg);
    msg
}
