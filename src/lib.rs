mod dns;
mod page;

use dns::{DnsInfo, HostInfo};
// use page::PageInfo;

#[derive(Debug, PartialEq, Clone)]
pub struct Domain(String);

impl Domain {
    pub fn clone(d: &Domain) -> Domain {
        Domain(String::from(&d.0))
    }
    pub fn from(s: &str) -> Option<Domain> {
        if s.contains('.') {
            Some(Domain(String::from(s.trim())))
        } else {
            None
        }
    }
}

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
    fn scan(&self) -> Res;
}

fn domain_scan(domain: &Domain) -> Option<DomainInfo> {
    if let Some(dns_info) = DnsInfo::from(domain) {
        let ip = dns_info.ip;
        let mut domain_info = DomainInfo::from(Domain::clone(domain), dns_info);
        domain_info.host_info = HostInfo::from(&ip);
        Some(domain_info)
    } else {
        None
    }
}

impl Scanner<Option<DomainInfo>> for Domain {
    fn scan(&self) -> Option<DomainInfo> {
        domain_scan(self)
    }
}
impl Scanner<Option<DomainInfo>> for str {
    fn scan(&self) -> Option<DomainInfo> {
        if let Some(domain) = Domain::from(self) {
            domain.scan()
        } else {
            None
        }
    }
}
impl Scanner<Vec<Option<DomainInfo>>> for Vec<Domain> {
    fn scan(&self) -> Vec<Option<DomainInfo>> {
        self.iter().map(|domain| domain.scan()).collect()
    }
}
impl Scanner<Vec<Option<DomainInfo>>> for Vec<&str> {
    fn scan(&self) -> Vec<Option<DomainInfo>> {
        self.iter()
            .map(|s| {
                if let Some(domain) = Domain::from(s) {
                    domain.scan()
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_from() {
        assert_eq!(None, Domain::from(""));
        assert_eq!(
            Some(Domain("www.google.com".to_string())),
            Domain::from("www.google.com")
        );
    }

    #[test]
    fn scanner() {
        assert_eq!(None, "".scan());
        assert_eq!(None, "".to_string().scan());
        assert_eq!(vec![None, None], vec!["", ""].scan());
        assert_eq!(
            vec![None, None],
            vec![Domain("".to_string()), Domain("".to_string())].scan()
        );
    }
}
