use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Result, Value};
use std::collections::HashMap;
use std::fs;

extern crate lazy_static;

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
    static ref TECHS: Vec<Tech> = {
        let mut techs:Vec<Tech> = vec![];
        for (app_name,app) in &APPS_JSON_DATA.apps {
            techs.push(Tech{name:String::from(app_name), category:app.category_name()})
        }
        techs
    };
}

pub struct Site {
    html: String,
}

impl Site {
    pub fn new(html: &str) -> Site {
        Site {
            html: String::from(html),
        }
    }
    pub fn check(&self) -> Vec<Tech> {
        vec![Tech::named("webpack").unwrap()]
    }
}

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
        assert!(self.cats.len() > 0);
        APPS_JSON_DATA.category_name(self.cats[0]).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    name: String,
    priority: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tech_lookup() {
        let tech = Tech::named("webpack").unwrap();
        assert_eq!(tech.name, "webpack");
        assert_eq!(tech.category, "Miscellaneous");
    }
}
