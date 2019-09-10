extern crate domain_info;
extern crate reqwest;

// use std::collections::HashMap;

use domain_info::{Domain, Scanner};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let domain_name = env::args()
        .nth(1)
        .ok_or("This command requires a domain as an argument")?;
    let domain: Domain = Domain::from(&domain_name)?;
    if let Ok(res) = match domain.scan() {
        Ok(info) => serde_json::to_string(&info),
        Err(err) => serde_json::to_string(&err),
    } {
        println!("{}", res);
    }
    Ok(())
}
