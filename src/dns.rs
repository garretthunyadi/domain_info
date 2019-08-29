use super::{Domain, Scanner};

#[derive(Debug, PartialEq)]
pub struct IP(String);

#[derive(Debug, PartialEq)]
pub struct DnsInfo {
    ip: IP,
}

#[derive(Debug, PartialEq)]
pub struct Host(String);

#[derive(Debug, PartialEq)]
pub enum HostPlatform {
    EigBluehost,
    EigHostgator,
    Google,
    Aws,
    GoDaddy,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct HostInfo {
    host: Host,
    platform: HostPlatform,
}

impl Scanner<Option<DnsInfo>> for Domain {
    fn scan(&self) -> Option<DnsInfo> {
        dns_lookup(self)
    }
}
impl Scanner<Option<HostInfo>> for IP {
    fn scan(&self) -> Option<HostInfo> {
        host_lookup(self)
    }
}

fn dns_lookup(_: &Domain) -> Option<DnsInfo> {
    Some(DnsInfo {
        ip: IP("0.0.0.0".to_string()),
    })
}

fn host_lookup(_: &IP) -> Option<HostInfo> {
    Some(HostInfo {
        host: Host("bogus".to_string()),
        platform: HostPlatform::Unknown,
    })
}
