extern crate domain_info;
extern crate reqwest;

// use std::collections::HashMap;

use domain_info::{Domain, Scanner};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let domain_name = env::args()
        .nth(1)
        .ok_or("This command requires a domain as an argument")?;
    println!("domain: {}", domain_name);
    let domain: Domain = Domain::from(&domain_name)?;
    let info = domain.scan();
    println!("{:?}", info);
    Ok(())
}
