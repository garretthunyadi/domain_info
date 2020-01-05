// extern crate domain_info;
// extern crate hyper;
extern crate reqwest;

// use hyper::Client;
// use std::{thread, time};

use domain_info::{Domain, DomainInfo, Scanner, ScannerResult};
// use std::env;
use std::io::{self, Read};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let domain_name = env::args()
//         .nth(1)
//         .ok_or("This command requires a domain as an argument")?;
//     let domain: Domain = Domain::from(&domain_name)?;
//     if let Ok(res) = match domain.scan() {
//         Ok(info) => serde_json::to_string(&info),
//         Err(err) => serde_json::to_string(&err),
//     } {
//         println!("{}", res);
//     }
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut buffer = String::new();
    // io::stdin().read_to_string(&mut buffer)?;

    // println!("==========\n{}\n=============", buffer);

    // let client = reqwest::Client::new();
    // let res = client.get("http://google.com").send().await?.text().await?;
    // println!("{:?}", res);

    //
    //
    //
    let res = Domain::from("cnn.com").unwrap().scan();
    eprintln!("\n\n\nres={:?}", res);

    //
    //
    //

    // let res = scann(Domain::from("google.com").unwrap());
    // let res = res.await;
    //
    //
    //
    //
    //
    // let domains = strings_to_domains(buffer).iter();
    // println!("{:?}", domains);
    // let domains = vec![Domain::from("google.com")];
    // let handles: Vec<tokio::task::JoinHandle<ScannerResult<DomainInfo>>> =
    //     domains.into_iter().map(|&d| d.scan()).collect();
    //     if let Ok(res) = match domain.scan() {
    //         Ok(info) => serde_json::to_string(&info),
    //         Err(err) => serde_json::to_string(&err),
    //     } {
    //         println!("{}", res);
    //     }

    // for handle in handles {
    //     let res = handle.await;
    //     println!("{:?}", res);
    // }
    Ok(())
}

fn strings_to_domains(domains: String) -> Vec<Domain> {
    domains
        .split_terminator('\n')
        .map(|s| Domain::from(s))
        .filter_map(Result::ok)
        .collect()
}

// fn start_proc() -> tokio::task::JoinHandle<String> {
//     tokio::spawn(async {
//         println!("Starting scan...");

//         let duration = 3000;

//         let pause_time = time::Duration::from_millis(duration);
//         let now = time::Instant::now();
//         thread::sleep(pause_time);
//         assert!(now.elapsed() >= pause_time);
//         // Return a value for the example
//         "result of the computation".to_string()
//     })
// }

fn scan(domain: Domain) -> tokio::task::JoinHandle<ScannerResult<DomainInfo>> {
    tokio::spawn(async move {
        domain.scan()
        //
        //
        //
        // if let Ok(res) = match domain.scan() {
        //     Ok(info) => serde_json::to_string(&info),
        //     Err(err) => serde_json::to_string(&err),
        // } {
        //     println!("{}", res);
        // }

        // let uri = format!("http://{}", domain).parse().unwrap();

        // let client = Client::new();

        // Parse an `http::Uri`...
        // let uri = "http://httpbin.org/ip".parse()?;
        // let uri = "http://httpbin.org/ip".parse().unwrap();

        // Await the response...
        // let resp = client.get(uri).await?;
        // let resp = client.get(uri).await.unwrap();

        // println!("Response: {}", resp.status());

        // use http::{Request, Response};

        // let mut request = Request::builder();
        // let uri = format!("https://{}", domain);
        // request.uri(uri);
        // let response = send(request.body(()).unwrap());
        // fn send(req: Request<()>) -> Response<()> {}
    })
}
// fn scan(domain_name: String) -> tokio::task::JoinHandle<String> {
//     tokio::spawn(async move {
//         if let Ok(domain) = Domain::from(&domain_name) {
//             if let Ok(res) = match domain.scan() {
//                 Ok(info) => serde_json::to_string(&info),
//                 Err(err) => serde_json::to_string(&err),
//             } {
//                 println!("{}", res);
//             }
//         }
//         // let uri = format!("http://{}", domain).parse().unwrap();

//         // let client = Client::new();

//         // Parse an `http::Uri`...
//         // let uri = "http://httpbin.org/ip".parse()?;
//         // let uri = "http://httpbin.org/ip".parse().unwrap();

//         // Await the response...
//         // let resp = client.get(uri).await?;
//         // let resp = client.get(uri).await.unwrap();

//         // println!("Response: {}", resp.status());

//         "OK!".to_string()
//         // use http::{Request, Response};

//         // let mut request = Request::builder();
//         // let uri = format!("https://{}", domain);
//         // request.uri(uri);
//         // let response = send(request.body(()).unwrap());
//         // fn send(req: Request<()>) -> Response<()> {}
//     })
// }
