//! # domain_info
//!
//! An early-stage crate and tool to fetch information about a domain, primarily by looking at the information on the front page of the domain.
//!
//! From the front page, it gets the load time, language, word count, image/form/script counts and it uses the Wappalizer project rules to identify technilogies used on the front page.
//!
//! It also does a reverse dns lookup on the host and attempts to determine the host company/platform (E.g GoDaddy, Bluehost, AWS)
//!
//! Additionally it gets the mail server hosts and whois information about the domain.

//!
//! ## Basic use
//!
//! For a single request:
//!
//! ```no_run
//! # use domain_info::{Domain,ScanError,Scanner};
//! let info = Domain::from("google.com").unwrap().scan();
//! ```
//!
//! (Note that this needs the tokio runtime)
//!
//! For a significant number of domains refer to the main.rs file for an example
//! which uses async/await and the tokio runtime to efficiently scan a list of domains.
//!
extern crate reqwest;
mod dns;
mod mx;
mod page;
mod wappalyzer;

#[macro_use]
extern crate lazy_static;

use dns::{DnsInfo, HostInfo};
use mx::MxInfo;
use page::PageInfo;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Possible Errors in the domain_info lib
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScanError {
    Domain(String),
    Dns(String),
    Other(String),
}
impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ScanError::Domain(err) => format!("Domain/{}", err),
                ScanError::Dns(err) => format!("Dns/{}", err),
                ScanError::Other(err) => format!("Other/{}", err),
            }
        )
    }
}
impl Error for ScanError {
    // fn description(&self) -> &str {
    //     &format!("{}", &self.to_string())
    // }
}

impl std::convert::From<std::io::Error> for ScanError {
    fn from(err: std::io::Error) -> Self {
        ScanError::Other(err.to_string())
    }
}
impl From<&dyn std::error::Error> for ScanError {
    fn from(err: &dyn std::error::Error) -> Self {
        ScanError::Other(err.to_string())
    }
}
// the trait `std::convert::From<page::reqwest::Error>` is not implemented for `ScanError`
impl From<reqwest::Error> for ScanError {
    fn from(err: reqwest::Error) -> Self {
        ScanError::Other(err.to_string())
    }
}
// the trait `std::convert::From<std::str::Utf8Error>` is not implemented for `ScanError`
impl From<std::str::Utf8Error> for ScanError {
    fn from(err: std::str::Utf8Error) -> Self {
        ScanError::Other(err.to_string())
    }
}

/// A wrapper type around a domain
///
/// # Examples
///
/// ```rust
/// # use domain_info::{Domain,ScanError};
/// assert_eq!(Domain::from("google.com"),Ok(Domain(String::from("google.com"))));
/// assert_eq!(Domain::from("invalid"),Err(ScanError::Domain(String::from("invalid domain"))));
/// ```
///
/// # Errors
///
/// This function fails if the domain is not in a valid form.
/// (TODO: add non-trivial domain validation.)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Domain(pub String);

impl Domain {
    pub fn from(s: &str) -> ScannerResult<Domain> {
        if s.contains('.') {
            Ok(Domain(String::from(s.trim())))
        } else {
            Err(ScanError::Domain("invalid domain".to_string()))
        }
    }
}

// impl Clone for Domain {
//     pub fn clone(d: &Domain) -> Domain {
//         Domain(String::from(&d.0))
//     }
// }

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Results of a scan
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainInfo {
    pub domain: Domain,
    pub dns_info: DnsInfo,
    pub host_info: Option<HostInfo>,
    // pub ssl_info: Option<SslInfo>,
    pub front_page_info: Option<PageInfo>,
    pub mx_info: Option<MxInfo>,
    // pub whois_info: Option<WhoisInfo>,
}

impl DomainInfo {
    pub fn from(domain: Domain, dns_info: DnsInfo) -> DomainInfo {
        DomainInfo {
            domain,
            dns_info,
            host_info: None,
            // ssl_info: None,
            front_page_info: None,
            mx_info: None,
            // whois_info: None,
            // crawl_info: None,
            // screenshot_info: None,
        }
    }
}

/// Helper type for the domain_info errors
pub type ScannerResult<T> = Result<T, ScanError>;

/// A very simple representation for cookie data
#[derive(Debug, PartialEq)]
pub struct Cookie {
    name: String,
    value: String,
}
// impl fmt::Display for DomainInfo {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             write!(f, "({}, {:?})", self.domain, self.dns_info)
//     }
// }

#[derive(Debug, PartialEq)]
pub struct SslInfo {}
#[derive(Debug, PartialEq)]
pub struct WhoisInfo {}

#[derive(Debug, PartialEq)]
pub struct CrawlInfo {}
#[derive(Debug, PartialEq)]
pub struct ScreenshotInfo {}

/// A helper trait to support ergonomic use of the lib
pub trait Scanner<Res> {
    fn scan(&self) -> ScannerResult<Res>;
}

async fn domain_scan(domain: &Domain) -> ScannerResult<DomainInfo> {
    let dns_info = DnsInfo::from(domain)?;
    let ip = dns_info.ip;
    let mut domain_info = DomainInfo::from(domain.clone(), dns_info);

    domain_info.host_info = HostInfo::from(&ip);
    domain_info.front_page_info = match page::front_page_scan(domain).await {
        Ok(page_info) => Some(page_info),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    };

    if let Ok(mx_info) = MxInfo::from(domain) {
        domain_info.mx_info = Some(mx_info);
    }

    Ok(domain_info)
}

impl Scanner<DomainInfo> for Domain {
    fn scan(&self) -> ScannerResult<DomainInfo> {
        let fut = domain_scan(self);
        use futures::executor::block_on;
        block_on(fut)
    }
}
impl Scanner<DomainInfo> for str {
    fn scan(&self) -> ScannerResult<DomainInfo> {
        let domain = Domain::from(self)?;
        domain.scan()
    }
}
// impl Scanner<Vec<DomainInfo>> for Vec<Domain> {
//     fn scan(&self) -> Vec<DomainInfo> {
//         self.iter().map(|domain| domain.scan()).collect()
//     }
// }
// impl Scanner<Vec<Option<DomainInfo>>> for Vec<&str> {
//     fn scan(&self) -> Vec<Option<DomainInfo>> {
//         self.iter()
//             .map(|s| {
//                 if let Some(domain) = Domain::from(s) {
//                     domain.scan()
//                 } else {
//                     None
//                 }
//             })
//             .collect()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_from() {
        assert_eq!(
            Err(ScanError::Domain("invalid domain".to_string())),
            Domain::from("")
        );
        assert_eq!(
            Ok(Domain("www.google.com".to_string())),
            Domain::from("www.google.com")
        );
    }

    #[test]
    fn scanner() {
        assert_eq!(
            Err(ScanError::Domain("invalid domain".to_string())),
            "".scan()
        );
        assert_eq!(
            Err(ScanError::Domain("invalid domain".to_string())),
            "".to_string().scan()
        );
        // assert_eq!(vec![None, None], vec!["", ""].scan());
        // assert_eq!(
        //     vec![None, None],
        //     vec![Domain("".to_string()), Domain("".to_string())].scan()
        // );
    }
}
