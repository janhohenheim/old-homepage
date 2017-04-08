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
use self::handlebars::{to_json, RenderError, RenderContext, Handlebars, Helper, Renderable};
use self::serde_json::Value;
use presentation::model::section::Section;
use std::collections::BTreeMap;
use presentation::helper::session::get_admin;




pub fn link_to_chain(chain: &mut Chain) -> Result<&mut Chain, SourceError> {
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./view/", ".hbs")));
    hbse.handlebars_mut().register_helper("if-eq", Box::new(if_eq));
    hbse.handlebars_mut().register_helper("if-mod", Box::new(if_mod));
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
        "parent".to_string() =>  to_json(&"layout/layout".to_string()),
    };
    base_data.append(&mut data);

    let admin = get_admin(req);
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



fn if_eq (h: &Helper, hbs: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    if_numeric_interaction(h, hbs, rc, |x, y| x == y)
}

fn if_mod (h: &Helper, hbs: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    if_numeric_interaction(h, hbs, rc, |x, y| (x % y) == 0)
}


fn if_numeric_interaction<F> (h: &Helper, hbs: &Handlebars, rc: &mut RenderContext, f: F) -> Result<(), RenderError>
    where F: Fn(i64, i64) -> bool {
    let param0 =
        h.param(0).ok_or_else(|| RenderError::new("First param not found for helper \"if_mod\""))?;
    let param1 =
        h.param(1).ok_or_else(|| RenderError::new("Second param not found for helper \"if_mod\""))?;

    let err_msg = "First param needs to be a number for helper \"if_mod\"";
    let nr0 = param0.value().as_i64().ok_or_else(|| RenderError::new(err_msg))?;
    let nr1 = param1.value().as_i64().ok_or_else(|| RenderError::new(err_msg))?;;
    let value = f(nr0, nr1);
    let tmpl = if value { h.template() } else { h.inverse() };
    match tmpl {
        Some(ref template) => template.render(hbs, rc),
        None => Ok(()),
    }
}