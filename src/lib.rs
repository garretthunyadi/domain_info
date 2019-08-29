mod dns;
mod page;

use dns::{DnsInfo, HostInfo};
use page::PageInfo;

#[derive(Debug, PartialEq)]
pub struct Domain(String);

impl Domain {
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
    host_info: HostInfo,
    ssl_info: SslInfo,
    front_page_info: PageInfo,
    mx_info: MxInfo,
    whois_info: WhoisInfo,

    crawl_info: CrawlInfo,
    screenshot_info: ScreenshotInfo,
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

fn domain_scan(_: &Domain) -> Option<DomainInfo> {
    None
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
