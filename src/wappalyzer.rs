use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Result, Value};
use std::collections::HashMap;
use std::fs;

pub fn load() -> Result<Apps> {
    let apps_json =
        fs::read_to_string("./apps.json").expect("Something went wrong reading the apps.json file");
    let apps: Apps = serde_json::from_str(&apps_json)?;
    Ok(apps)
}

pub struct Site {
    html: String,
}

impl Site {
    fn check(&self) -> Vec<Tech> {
        vec![]
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tech {
    category: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Apps {
    apps: HashMap<String, App>,
    categories: HashMap<u32, Category>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct App {
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
    fn check() {}
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    name: String,
    priority: u8,
}
