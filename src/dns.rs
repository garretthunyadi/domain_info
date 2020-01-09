extern crate dns_lookup;
use super::{Domain, ScanError, ScannerResult};
use serde::{Deserialize, Serialize};

/// Basic DNS lookup infomation, including the ip and
/// saving additional ips as a vector.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsInfo {
    pub ip: std::net::IpAddr,
    pub other_ips: Vec<std::net::IpAddr>,
}

impl DnsInfo {
    pub fn from(domain: &Domain) -> ScannerResult<DnsInfo> {
        dns_lookup(domain)
    }
}

// impl Scanner<Option<DnsInfo>> for Domain {
//     fn scan(&self) -> Option<DnsInfo> {
//         dns_lookup(self)
//     }
// }

// impl Scanner<DnsInfo> for Domain {
//     fn scan(&self) -> DnsInfo {
//         if let Some(dns_info) = dns_lookup(self) {
//             dns_info
//         } else { DnsInfo{ips:vec![]} }
//     }
// }

/// Information from a dns scan, including a reverse lookup of the server,
/// hopefully finding the company/brand that hosts the site.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HostInfo {
    pub host: Host,
    pub host_tld: HostTld,
    pub platform: HostPlatform,
}

impl HostInfo {
    pub fn from(ip: &std::net::IpAddr) -> Option<HostInfo> {
        host_lookup(ip)
    }
}

// impl Scanner<Option<HostInfo>> for std::net::IpAddr {
//     fn scan(&self) -> Option<HostInfo> {
//         host_lookup(self)
//     }
// }

/// A website's hostname. E.g. server51.hostgator.com
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Host(String);

/// The top level domain for the host. E.g. hostgator.com
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HostTld(String);

/// Known list of host platforms (this is who is hosting the website)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HostPlatform {
    A2Hosting,
    AmazonAws,
    Datafoundry,
    Dreamhost,
    EigASmallOrange,
    EigBluehost,
    EigDomainDotCom,
    EigFastDomain,
    EigHostgator,
    EigHostmonster,
    EigIpage,
    EigJustHost,
    Eigverio,
    Godaddy,
    GodaddySucuri,
    Google,
    IBMCloud,
    IncapsulaCdn,
    Inmotionhosting,
    Ionos1And1,
    Linode,
    Namecheap,
    NetworkSolutions,
    SiteGround,
    Tucows,
    TucowsHover,
    Webair,
    Weebly,
    Yahoo,
    Unknown,
}

impl HostPlatform {
    fn from(tld: &str) -> HostPlatform {
        use HostPlatform::*;
        match tld {
            "1e100.net" => Google,
            "a2hosting.com" => A2Hosting,
            "amazonaws.com" => AmazonAws,
            "asoshared.com" => EigASmallOrange,
            "bluehost.com" => EigBluehost,
            "callbuttonsource.com" => Webair,
            "datafoundry.com" => Datafoundry,
            "dreamhost.com" => Dreamhost,
            "eigbox.net" => EigIpage,
            "fastdomain.com" => EigFastDomain,
            "googleusercontent.com" => Google,
            "hostgator.com" => EigHostgator,
            "hostmonster.com" => EigHostmonster,
            "hover.com" => TucowsHover,
            "incapdns.net" => IncapsulaCdn,
            "inmotionhosting.com" => Inmotionhosting,
            "justhost.com" => EigJustHost,
            "linode.com" => Linode,
            "managednsservice.com" => Tucows,
            "netsolhost.com" => NetworkSolutions,
            "networksolutions.com" => NetworkSolutions,
            "secureserver.net" => Godaddy,
            "siteground.com" => SiteGround,
            "sl-reverse.com" => IBMCloud,
            "sucuri.net" => GodaddySucuri,
            "ui-r.com" => Ionos1And1,
            "unifiedlayer.com" => EigBluehost,
            "verio.com" => Eigverio,
            "web-hosting.com" => Namecheap,
            "webhostbox.net" => EigIpage,
            "websitehostserver.net" => Godaddy,
            "weebly.com" => Weebly,
            "yahoo.com" => Yahoo,
            "yourhostingaccount.com" => Tucows,
            "yourwebhosting.com" => EigDomainDotCom,
            _ => Unknown,
        }
    }
}

fn dns_lookup(domain: &Domain) -> ScannerResult<DnsInfo> {
    let ips = dns_lookup::lookup_host(&domain.0)
        .or_else(|_| Err(ScanError::Dns("couldn't lookup".to_string())))?;
    let (first, rest) = ips
        .split_first()
        .ok_or_else(|| ScanError::Dns("no ips found (I think)".to_string()))?;
    Ok(DnsInfo {
        ip: *first,
        other_ips: rest.to_vec(),
    })
}

fn host_lookup(ip: &std::net::IpAddr) -> Option<HostInfo> {
    if let Ok(host) = dns_lookup::lookup_addr(ip) {
        let tld = tld_for_host(&host);
        Some(HostInfo {
            host: Host(String::from(&host)),
            host_tld: HostTld(String::from(&tld)),
            platform: HostPlatform::from(&tld),
        })
    } else {
        None
    }
}

fn tld_for_host(host: &str) -> String {
    let mut iter = host.split('.');
    let last = iter.next_back();
    let second_from_last = iter.next_back();
    let third_from_last = iter.next_back();

    match (third_from_last, second_from_last, last) {
        (Some(a), Some("co"), Some("uk")) => format!("{}.co.uk", a),
        (Some(_), Some(a), Some(b)) => format!("{}.{}", a, b),
        _ => host.to_string(), // default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dns_lookup_crate_expectations_lookup_host() {
        let hostname = "localhost";
        let ips: Vec<std::net::IpAddr> = dns_lookup::lookup_host(hostname).unwrap();
        // println!("{:?}", ips);
        assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
    }

    #[test]
    fn dns_lookup_crate_expectations_lookup_addr() {
        let ip: std::net::IpAddr = "127.0.0.1".parse().unwrap();
        let host = dns_lookup::lookup_addr(&ip).unwrap();
        assert_eq!("localhost".to_string(), host);
    }

    #[test]
    fn my_dns_lookup() {
        if dns_lookup(&Domain("google.com".to_string())).is_err() {
            panic!();
        }
    }

    #[test]
    fn my_host_lookup() {
        // first get the ip for google.com
        if let Ok(dns_info) = dns_lookup(&Domain("google.com".to_string())) {
            let google_ip = dns_info.ip;

            // and then get a host
            if let Some(host_info) = host_lookup(&google_ip) {
                assert_eq!(HostPlatform::Google, host_info.platform);
                // bom07s20-in-f14.1e100.net
                // lga34s12-in-f14.1e100.net
                assert!(host_info.host.0.contains("1e100.net"))
            } else {
                unreachable!();
            }
        }
    }

    #[test]
    fn test_tld_for_host() {
        assert_eq!("google.com".to_string(), tld_for_host("google.com"));
        assert_eq!("google.com".to_string(), tld_for_host("foo.google.com"));
        assert_eq!("google.co.uk".to_string(), tld_for_host("foo.google.co.uk"));
    }
}
