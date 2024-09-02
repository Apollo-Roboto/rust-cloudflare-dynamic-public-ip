use core::fmt;
use std::{net::Ipv4Addr, str::FromStr};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[allow(dead_code)]
pub enum CloudFlareClientError {
    Request(reqwest::Error),
    Api(ErrorResponse),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ErrorResponse {
    pub errors: Vec<Message>,
    pub messages: Vec<Message>,
    pub success: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SuccessResponseList<T> {
    pub errors: Vec<Message>,
    pub messages: Vec<Message>,
    pub success: bool,
    pub result_info: ResultInfo,
    pub result: Vec<T>,
}

impl<T> SuccessResponseList<T> {
    pub fn count(&self) -> usize {
        self.result.len()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UpdateDNSRecordRequest {
    pub content: String,
    pub name: String,
    pub proxied: Option<bool>,
    pub r#type: DNSType,
    pub comment: Option<String>,
    pub id: String,
    pub tags: Option<Vec<String>>,
    pub ttl: Option<i32>,
}

impl From<DNSRecord> for UpdateDNSRecordRequest {
    fn from(value: DNSRecord) -> Self {
        Self {
            content: value.content,
            name: value.name,
            proxied: value.proxied,
            r#type: value.r#type,
            comment: value.comment,
            id: value.id,
            tags: value.tags,
            ttl: value.ttl,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub code: i32,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResultInfo {
    pub count: i32,
    pub page: i32,
    pub per_page: i32,
    pub total_count: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DNSRecord {
    pub content: String,
    pub name: String,
    pub proxied: Option<bool>,
    pub r#type: DNSType,
    pub comment: Option<String>,
    pub comment_modified_on: Option<DateTime<Utc>>,
    pub created_on: DateTime<Utc>,
    pub id: String,
    pub meta: Option<DNSRecordMeta>,
    pub modified_on: DateTime<Utc>,
    pub proxiable: bool,
    pub tags: Option<Vec<String>>,
    pub tags_modified_on: Option<DateTime<Utc>>,
    pub ttl: Option<i32>,
}

impl DNSRecord {
    pub fn has_tags(&self) -> bool {
        match &self.tags {
            Some(tags) => !tags.is_empty(),
            None => false,
        }
    }

    pub fn content_as_ip(&self) -> Result<Ipv4Addr, std::net::AddrParseError> {
        Ipv4Addr::from_str(&self.content)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DNSType {
    A,
    AAAA,
    CAA,
    CERT,
    CNAME,
    DNSKEY,
    DS,
    HTTPS,
    LOC,
    MX,
    NAPTR,
    NS,
    PTR,
    SMIMEA,
    SRV,
    SSHFP,
    SVCB,
    TLSA,
    TXT,
    URI,
}

impl fmt::Display for DNSType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DNSType {
    /// https://en.wikipedia.org/wiki/List_of_DNS_record_types
    pub fn id(&self) -> i32 {
        match self {
            DNSType::A => 1,
            DNSType::AAAA => 28,
            DNSType::CAA => 257,
            DNSType::CERT => 37,
            DNSType::CNAME => 5,
            DNSType::DNSKEY => 48,
            DNSType::DS => 43,
            DNSType::HTTPS => 65,
            DNSType::LOC => 29,
            DNSType::MX => 15,
            DNSType::NAPTR => 35,
            DNSType::NS => 2,
            DNSType::PTR => 12,
            DNSType::SMIMEA => 53,
            DNSType::SRV => 33,
            DNSType::SSHFP => 44,
            DNSType::SVCB => 64,
            DNSType::TLSA => 52,
            DNSType::TXT => 16,
            DNSType::URI => 256,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DNSRecordMeta {
    pub auto_added: bool,
    pub source: Option<String>,
}
