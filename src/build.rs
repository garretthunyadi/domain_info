extern crate reqwest;

use std::{fs::File, io::Write, path::Path};

const APPS_JSON_PATH: &str = "./apps.json";
const WAPPALYZER_APPS_JSON_URL: &str =
    "https://raw.githubusercontent.com/AliasIO/wappalyzer/master/src/apps.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(APPS_JSON_PATH).exists() {
        println!("DOWNLOADING apps.json");
        let response = reqwest::get(WAPPALYZER_APPS_JSON_URL)
            .await
            .unwrap()
            .text()
            .await
            .unwrap(); // TODO
        let mut dest = File::create(APPS_JSON_PATH)?;
        write!(&mut dest, "{}", response)?;
    }
    Ok(())
}
