extern crate iron;
extern crate mime;
extern crate router;
extern crate handlebars;
extern crate handlebars_iron as hbs;
extern crate serde;
extern crate serde_json;

use self::iron::Request;
use self::iron::prelude::*;
use self::hbs::{Template, HandlebarsEngine, DirectorySource, SourceError};
use self::handlebars::to_json;
use self::serde_json::Value;
use session;
use std::collections::BTreeMap;

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

pub fn generate_site(req: &mut Request,
                     path: &str,
                     mut data: BTreeMap<String, Value>,
                     section: Option<&Section>)
                     -> Template {
    let mut sections = get_sections();
    if let Some(section) = section {
        set_active_section(&mut sections, section);
    }
    let mut base_data = btreemap! {
        "sections".to_string() => to_json(&sections),
        "parent".to_string() =>  to_json(&"template".to_string()),
    };
    base_data.append(&mut data);

    let admin = session::get_admin(req);
    if let Ok(admin_ok) = admin {
        if let Some(admin_some) = admin_ok {
            let mut username = btreemap! {
                "username".to_owned() => to_json(&admin_some.name)
            };
            base_data.append(&mut username);
        }
    }

    Template::new(path, base_data)
}

pub fn generate_site_without_data(req: &mut Request,
                                  path: &str,
                                  section: Option<&Section>)
                                  -> Template {
    generate_site(req, path, BTreeMap::new(), section)
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

fn set_active_section(sections: &mut Vec<SectionData>, active: &Section) {
    for section in sections {
        if section.name == *active {
            section.is_active = true
        }
    }
}
