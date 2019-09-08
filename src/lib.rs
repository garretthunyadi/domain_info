mod dns;
mod page;

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ScanError {
    Domain(String),
    Dns(String),
    Content(String),
    Core(String),
    Head(String),
    Other(String),
}
impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}
impl Error for ScanError {
    fn description(&self) -> &str {
        "I'm the superhero of errors"
    }
}

// std::convert::From<std::io::Error>
impl std::convert::From<std::io::Error> for ScanError {
    fn from(ioe: std::io::Error) -> Self {
        ScanError::Other(ioe.to_string())
    }
}

// use std::fmt;
use dns::{DnsInfo, HostInfo};
// use page::PageInfo;

#[derive(Debug, PartialEq, Clone)]
pub struct Domain(String);

impl Domain {
    pub fn clone(d: &Domain) -> Domain {
        Domain(String::from(&d.0))
    }
    pub fn from(s: &str) -> ScannerResult<Domain> {
        if s.contains('.') {
            Ok(Domain(String::from(s.trim())))
        } else {
            Err(ScanError::Domain("invalid domain".to_string()))
        }
    }
}
// impl fmt::Display for Domain {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             write!(f, "Domain({})", self.0)
//     }
// }

#[derive(Debug, PartialEq)]
pub struct DomainInfo {
    domain: Domain,
    dns_info: DnsInfo,
    host_info: Option<HostInfo>,
    ssl_info: Option<SslInfo>,
    // front_page_info: Option<PageInfo>,
    // mx_info: Option<MxInfo>,
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
            ssl_info: None,
            // front_page_info: None,
            // mx_info: None,
            // whois_info: None,
            // crawl_info: None,
            // screenshot_info: None,
        }
    }
}

// impl fmt::Display for DomainInfo {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             write!(f, "({}, {:?})", self.domain, self.dns_info)
//     }
// }

#[derive(Debug, PartialEq)]
pub struct SslInfo {}
#[derive(Debug, PartialEq)]
pub struct MxInfo {}
#[derive(Debug, PartialEq)]
pub struct WhoisInfo {}

#[derive(Debug, PartialEq)]
pub struct CrawlInfo {}
#[derive(Debug, PartialEq)]
pub struct ScreenshotInfo {}

pub trait Scanner<Res> {
    fn scan(&self) -> ScannerResult<Res>;
}

fn domain_scan(domain: &Domain) -> ScannerResult<DomainInfo> {
    let dns_info = DnsInfo::from(domain)?;
    let ip = dns_info.ip;
    let mut domain_info = DomainInfo::from(Domain::clone(domain), dns_info);
    domain_info.host_info = HostInfo::from(&ip);
    Ok(domain_info)
    // if let Ok(dns_info) = DnsInfo::from(domain) {
    //     let ip = dns_info.ip;
    //     let mut domain_info = DomainInfo::from(Domain::clone(domain), dns_info);
    //     domain_info.host_info = HostInfo::from(&ip);
    //     Ok(domain_info)
    // } else {
    //     Err()
    // }
}

impl Scanner<DomainInfo> for Domain {
    fn scan(&self) -> ScannerResult<DomainInfo> {
        domain_scan(self)
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
