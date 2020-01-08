extern crate reqwest;

use domain_info::{Domain, Scanner};
use futures::future::join_all;
use std::io::{self, Read};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let domains = strings_to_domains(buffer);
    println!("{:?}", domains);

    let futures = domains
        .into_iter()
        .map(|d| async move { Domain(d.0).scan() })
        .collect::<Vec<_>>();
    println!("{:?},", join_all(futures).await);

    Ok(())
}

fn strings_to_domains(domains: String) -> Vec<Domain> {
    domains
        .split_terminator('\n')
        .map(|s| Domain::from(s))
        .filter_map(Result::ok)
        .collect()
}
