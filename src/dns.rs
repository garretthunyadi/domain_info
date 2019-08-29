extern crate dns_lookup;
use super::Domain;

#[derive(Debug, PartialEq)]
pub struct DnsInfo {
    pub ip: std::net::IpAddr,
    pub other_ips: Vec<std::net::IpAddr>,
}

impl DnsInfo {
    pub fn from(domain:&Domain) -> Option<DnsInfo> {
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

#[derive(Debug, PartialEq)]
pub struct HostInfo {
    pub host: Host,
    pub host_tld: HostTld,
    pub platform: HostPlatform,
}

impl HostInfo {
    pub fn from(ip:&std::net::IpAddr) -> Option<HostInfo> {
        host_lookup(ip)
    }
}

// impl Scanner<Option<HostInfo>> for std::net::IpAddr {
//     fn scan(&self) -> Option<HostInfo> {
//         host_lookup(self)
//     }
// }

#[derive(Debug, PartialEq)]
pub struct Host(String);
#[derive(Debug, PartialEq)]
pub struct HostTld(String);

#[derive(Debug, PartialEq)]
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


// TODO: should change this to use result
fn dns_lookup(domain: &Domain) -> Option<DnsInfo> {
    if let Ok(ips) = dns_lookup::lookup_host(&domain.0) {
        if let Some((first,rest)) = ips.split_first() {
            Some(DnsInfo { ip: *first, other_ips: rest.to_vec() })
        } else {
            None
        }
    } else {
        None
    }
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
        if let Some(dns_info) = dns_lookup(&Domain("google.com".to_string())) {
            assert!(!dns_info.ips.is_empty());
        } else {
            unreachable!();
        }
    }

    #[test]
    fn my_host_lookup() {
        // first get the ip for google.com
        if let Some(dns_info) = dns_lookup(&Domain("google.com".to_string())) {
            let google_ip = dns_info.ip;

            // and then get a host
            if let Some(host_info) = host_lookup(&google_ip) {
                assert_eq!(HostPlatform::Google, host_info.platform);
                assert_eq!(
                    Host("bom07s20-in-f14.1e100.net".to_string()),
                    host_info.host
                );
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
