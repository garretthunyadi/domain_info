extern crate reqwest;

use domain_info::{Domain, Scanner};
use futures::future::join_all;
use std::env;
use std::io::{self, Read};

macro_rules! s {
    ($e:expr) => {
        String::from($e)
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut domains = vec![];
    if args.len() == 1 {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        domains.extend(strings_to_domains(buffer));
    } else {
        domains.push(Domain(s![&args[1]]));
    }

    let futures = domains
        .into_iter()
        .map(|d| async move { Domain(d.0).scan() })
        .collect::<Vec<_>>();
    let results = join_all(futures).await;
    for res in results {
        if let Ok(output) = match res {
            Ok(info) => serde_json::to_string(&info),
            Err(err) => serde_json::to_string(&err),
        } {
            println!("{}", output);
        }
    }
    Ok(())
}

fn strings_to_domains(domains: String) -> Vec<Domain> {
    domains
        .split_terminator('\n')
        .map(|s| Domain::from(s))
        .filter_map(Result::ok)
        .collect()
}
