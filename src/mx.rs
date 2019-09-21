use std::str::FromStr;
use trust_dns::client::{Client, SyncClient};
use trust_dns::op::DnsResponse;
use trust_dns::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns::udp::UdpClientConnection;

use super::{Domain, ScannerResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MxInfo {
    pub servers: Vec<String>,
}

impl MxInfo {
    pub fn from(domain: &Domain) -> ScannerResult<MxInfo> {
        mx_lookup(domain)
    }
}

fn mx_lookup(domain: &Domain) -> ScannerResult<MxInfo> {
    let address = "8.8.8.8:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);

    let name = Name::from_str(&domain.0).unwrap();
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::MX).unwrap();
    let answers: &[Record] = response.answers();

    // eprintln!("{:?}", answers);
    let mx_servers = answers
        .iter()
        .map(|answer| {
            if let RData::MX(ref mx) = answer.rdata() {
                let server = mx.exchange().to_lowercase().to_ascii();
                // strip off the last '.'
                if server.ends_with('.') {
                    server[0..server.len() - 1].to_string()
                } else {
                    server
                }
            } else {
                "mx_error".to_string()
            }
        })
        .collect();

    Ok(MxInfo {
        servers: mx_servers,
    })
}
