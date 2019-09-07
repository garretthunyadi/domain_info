extern crate domain_info;
extern crate reqwest;

// use std::collections::HashMap;

use domain_info::{Domain, Scanner};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("{}", env::args().nth(1).ok_or("Missing argument")?);
//     let resp = reqwest::get("http://httpbin.org/get")?;
//     if resp.status().is_success() {
//         println!("success!");
//     } else if resp.status().is_server_error() {
//         println!("server error!");
//     } else {
//         println!("Something else happened. Status: {:?}", resp.status());
//     }
//     // many_failures()?
//     Ok(())
// }
fn main() -> io::Result<()> {
    // Scan one domain
    if let Ok(domain_name) = env::args().nth(1).ok_or("Missing argument") {
        println!("domain: {}", domain_name);
        if let Some(domain) = Domain::from(&domain_name) {
            if let Some(dinfo) = domain.scan() {
                println!("{:?}",dinfo);
                return Ok(())
            }
        }
        // if let Some(domain_info) = domain.scan() {
        // }

        // return Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
        // }
        // println!(
        //     "{}",
        //     match many_failures(1) {
        //         Ok(()) => "",
        //         Err(_) => "ugg! #55583",
        //     }
        // );
    }

    // println!("{}", env::args().nth(1).ok_or("Missing argument")?);
    Err(Error::new(ErrorKind::Other, "Couldn't Scan"))
}

// use std::fmt;
// use std::io;
// enum InSitesError {
//     // IoError(io::Error),
//     IDontWantToWork,
//     // NotMyJob,
//     // Other(String),
// }

// impl fmt::Display for InSitesError {
//     fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//         formatter.write_str("TODO/Error formatting")
//     }
// }

// fn many_failures(x: u16) -> Result<(), InSitesError> {
//     match x {
//         0 => Ok(()),
//         _ => Err(InSitesError::IDontWantToWork),
//     }
// }

// fn main() -> Result<(),Box<std::io::Error>> {
//     // if let Ok(resp) = reqwest::get("https://httpbin.org/ip") {
//             // }

//     // https://docs.rs/reqwest/0.9.19/reqwest/struct.Response.html
// let resp = reqwest::get("http://httpbin.org/get")?;
// if resp.status().is_success() {
//     println!("success!");
// } else if resp.status().is_server_error() {
//     println!("server error!");
// } else {
//     println!("Something else happened. Status: {:?}", resp.status());
// }

// let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?.json()?;
// println!("{:#?}", resp);
// Ok(())
// }

// use domain_info::{Domain, Scanner};
// use std::*;

// fn main2() {
//     let domains = read_domains_from_stdin();
//     let domain_infos = domains.scan();

//     for info in domain_infos {
//         println!("{:?}", info);
//     }
// }

// fn read_domains_from_stdin() -> Vec<Domain> {
//     let mut domains: Vec<Domain> = vec![];
//     let stdin = io::stdin();
//     let line = &mut String::new();

//     loop {
//         line.clear();
//         stdin.read_line(line).expect("=========fail============");

//         if line.is_empty() {
//             break;
//         }
//         if let Some(domain) = Domain::from(line) {
//             domains.push(domain);
//         } else {
//             println!("(bad domain: '{}'", line);
//         }
//     }
//     domains
// }
