/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TlsDnsRecord {
    /// Specifies the regions that will be used to route traffic. Select DNS records with a `global` region to route traffic to the most performant point of presence (POP) worldwide (global pricing will apply). Select DNS records with a `na/eu` region to exclusively land traffic on North American and European POPs.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<Region>,
    /// The type of the DNS record. `A` specifies an IPv4 address to be used for an A record to be used for apex domains (e.g., `example.com`). `AAAA` specifies an IPv6 address for use in an A record for apex domains. `CNAME` specifies the hostname to be used for a CNAME record for subdomains or wildcard domains (e.g., `www.example.com` or `*.example.com`).
    #[serde(rename = "record_type", skip_serializing_if = "Option::is_none")]
    pub record_type: Option<RecordType>,
}

impl TlsDnsRecord {
    pub fn new() -> TlsDnsRecord {
        TlsDnsRecord {
            region: None,
            record_type: None,
        }
    }
}

/// Specifies the regions that will be used to route traffic. Select DNS records with a `global` region to route traffic to the most performant point of presence (POP) worldwide (global pricing will apply). Select DNS records with a `na/eu` region to exclusively land traffic on North American and European POPs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "na/eu")]
    NaEu,
}

impl Default for Region {
    fn default() -> Region {
        Self::Custom
    }
}
/// The type of the DNS record. `A` specifies an IPv4 address to be used for an A record to be used for apex domains (e.g., `example.com`). `AAAA` specifies an IPv6 address for use in an A record for apex domains. `CNAME` specifies the hostname to be used for a CNAME record for subdomains or wildcard domains (e.g., `www.example.com` or `*.example.com`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecordType {
    #[serde(rename = "CNAME")]
    CNAME,
    #[serde(rename = "A")]
    A,
    #[serde(rename = "AAAA")]
    AAAA,
}

impl Default for RecordType {
    fn default() -> RecordType {
        Self::CNAME
    }
}

