use domain_info::{Domain, Scanner};
use std::*;

fn main() {
    let domains = read_domains_from_stdin();
    let domain_infos = domains.scan();

    for info in domain_infos {
        println!("{:?}", info);
    }
}

fn read_domains_from_stdin() -> Vec<Domain> {
    let mut domains: Vec<Domain> = vec![];
    let stdin = io::stdin();
    let line = &mut String::new();

    loop {
        line.clear();
        stdin.read_line(line).expect("=========fail============");

        if line.is_empty() {
            break;
        }
        if let Some(domain) = Domain::from(line) {
            domains.push(domain);
        } else {
            println!("(bad domain: '{}'", line);
        }
    }
    domains
}
