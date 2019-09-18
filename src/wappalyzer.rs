use serde::{Deserialize, Serialize};
// use serde_json::{json, Map, Result, Value};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

extern crate lazy_static;

pub fn check(headers: &reqwest::header::HeaderMap, body: &str) -> Vec<Tech> {
    APPS_JSON_DATA
        .apps
        .iter()
        .filter_map(|(_name, app)| app.tech(headers, body))
        .collect()
}

lazy_static! {
    static ref APPS_JSON_DATA: AppsJsonData = {
        let apps_json = fs::read_to_string("./apps.json")
            .expect("Something went wrong reading the apps.json file");
        let mut apps_json_data:AppsJsonData = serde_json::from_str(&apps_json).expect("Error loading the apps.json file");

        // for (app_name,&mut app) in &apps_json_data.apps.iter_mut() {
        //     app.
        // }
        for (app_name, app) in apps_json_data.apps.iter_mut() {
            (*app).name = String::from(app_name);
        }

        apps_json_data
    };
    // static ref APP_NAME_LOOKUP: HashMap<App,String> = {
    //     let mut app_name_lookup:HashMap<App,&str> = HashMap::new();
    //     for (app_name,app) in &APPS.apps {
    //         app_name_lookup.insert(app, app_name);
    //     }
    //     app_name_lookup
    // };
    // static ref TECHS: Vec<Tech> = {
    //     let mut techs:Vec<Tech> = vec![];
    //     for (app_name,app) in &APPS_JSON_DATA.apps {
    //         techs.push(Tech{name:String::from(app_name), category:app.category_name()})
    //     }
    //     techs
    // };
}

// pub struct Site {
//     html: String,
// }

// impl Site {
//     pub fn new(html: &str) -> Site {
//         Site {
//             html: String::from(html),
//         }
//     }
//     pub fn check(&self) -> Vec<Tech> {
//         // let mut techs = vec![];
//         // for (_name, app) in APPS_JSON_DATA.apps.iter() {
//         //     if let Some(tech) = app.tech(&self.html) {
//         //         techs.push(tech);
//         //     }
//         // }

//         APPS_JSON_DATA
//             .apps
//             .iter()
//             .filter_map(|(_name, app)| app.tech(&self.html))
//             .collect()
//         // let mut iter = a.iter().filter_map(|s| s.parse().ok());

//         // vec![Tech::named("webpack").unwrap()]
//         // techs
//     }
// }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tech {
    category: String,
    name: String,
}
impl Tech {
    /// let tech = Tech::named("webpack");
    /// assert_eq!(tech.name, "webpack");
    /// assert_eq!(tech.category, "Miscellaneous");
    fn named(name: &str) -> Option<Tech> {
        if let Some(app) = APPS_JSON_DATA.named(name) {
            Some(Tech::from(app))
        } else {
            None
        }
    }

    pub fn from(app: &App) -> Tech {
        Tech {
            name: app.name.clone(),
            category: app.category_name(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppsJsonData {
    apps: HashMap<String, App>,
    categories: HashMap<u32, Category>,
}
impl AppsJsonData {
    fn named(&self, name: &str) -> Option<&App> {
        self.apps.get(&String::from(name))
    }

    fn category_name(&self, id: u32) -> Option<String> {
        match self.categories.get(&id) {
            // Some(category) => Some(String::from(category.name)),
            Some(category) => Some(category.name.clone()),
            None => None,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct App {
    #[serde(skip)]
    name: String,
    cats: Vec<u32>,
    website: String,
    #[serde(default)]
    priority: i32,
    #[serde(default)]
    headers: HashMap<String, String>,
    #[serde(default)]
    cookies: HashMap<String, String>,
    #[serde(default)]
    js: HashMap<String, String>,
    #[serde(default)]
    url: String,
    #[serde(default)]
    meta: HashMap<String, String>,
    #[serde(default)]
    icon: String,
    #[serde(default)]
    implies: Value,
    #[serde(default)]
    excludes: Value,
    #[serde(default)]
    script: Value,
}

impl App {
    pub fn category_name(&self) -> String {
        assert!(!self.cats.is_empty());
        APPS_JSON_DATA.category_name(self.cats[0]).unwrap()
    }

    // pub fn check_headers(&self,)
    pub fn tech(&self, headers: &reqwest::header::HeaderMap, html: &str) -> Option<Tech> {
        if self.check(headers, html) {
            Some(Tech::from(self))
        } else {
            None
        }
    }

    // TODO: initially only checking for one positive
    pub fn check(&self, headers: &reqwest::header::HeaderMap, html: &str) -> bool {
        // check headers
        for (header_to_check, expected_value) in self.headers.iter() {
            if let Some(value) = headers.get(header_to_check) {
                // println!("1. {:?}", value);
                if let Ok(string_value) = value.to_str() {
                    if check_text(expected_value, string_value) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    name: String,
    priority: u8,
}

// The meat of the matter
fn check_text(maybe_regex: &str, text: &str) -> bool {
    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    match Regex::new(maybe_regex) {
        Ok(re) => {
            // println!("REGEX IS FINE: [{}]", maybe_regex);

            if re.is_match(text) {
                // println!("MATCH! [{}] on text '{}'", maybe_regex, text);
                true
            } else {
                false
            }
        }
        Err(err) => {
            // println!("invalid regex in app.json '{}': {}", maybe_regex, err);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header;

    #[test]
    fn tech_lookup() {
        let tech = Tech::named("webpack").unwrap();
        assert_eq!(tech.name, "webpack");
        assert_eq!(tech.category, "Miscellaneous");
    }

    #[test]
    fn test_check_app() {
        assert_eq!(
            APPS_JSON_DATA
                .named("webpack")
                .unwrap()
                .tech(&header::HeaderMap::new(), ""),
            None
        );
        // assert_eq!(
        //     APPS_JSON_DATA.named("webpack").unwrap().tech(""),
        //     Tech::named("webpack")
        // );
    }

    #[test]
    fn test_check_text() {
        assert!(check_text("foo", "somefood"));
        assert!(!check_text("bar", "somefood"));
        assert!(check_text("[CK]amva", "Kamva"));
        assert!(!check_text("[CK]amva", "Lamva"));
        assert!(check_text(
            "cf\\.kampyle\\.com/k_button\\.js",
            "some cf.kampyle.com/k_button.js"
        ));
        assert!(!check_text(
            "cf\\.kampyle\\.com/k_button\\.js",
            "some cXf.kampyle.com/k_button.js"
        ));
        // assert!(!check_text(
        //     "<link[^>]*\\s+href=[^>]*styles/kendo\\.common(?:\\.min)?\\.css[^>]*/>",
        //     ""
        // ));
        // assert!(check_text(
        //     "<link[^>]*\\s+href=[^>]*styles/kendo\\.common(?:\\.min)?\\.css[^>]*/>",
        //     "<link "
        // ));
    }
}
