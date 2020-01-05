extern crate reqwest;
mod dns;
mod mx;
mod page;
mod wappalyzer;

#[macro_use]
extern crate lazy_static;

use std::error::Error;
use std::fmt;
// use std::fmt;
use dns::{DnsInfo, HostInfo};
use mx::MxInfo;
use page::PageInfo;
// use page::PageInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScanError {
    Domain(String),
    Dns(String),
    Content(String),
    Page(String),
    Head(String),
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
                ScanError::Content(err) => format!("Content/{}", err),
                ScanError::Page(err) => format!("Page/{}", err),
                ScanError::Head(err) => format!("Head/{}", err),
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Domain(String);

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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainInfo {
    domain: Domain,
    dns_info: DnsInfo,
    host_info: Option<HostInfo>,
    // ssl_info: Option<SslInfo>,
    front_page_info: Option<PageInfo>,
    mx_info: Option<MxInfo>,
    // whois_info: Option<WhoisInfo>,

    // crawl_info: Option<CrawlInfo>,
    // screenshot_info: Option<ScreenshotInfo>,
}

pub type ScannerResult<T> = Result<T, ScanError>;

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
