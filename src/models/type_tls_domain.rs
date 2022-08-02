/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// TypeTlsDomain : Resource type

/// Resource type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeTlsDomain {
    #[serde(rename = "tls_domain")]
    TlsDomain,

}

impl ToString for TypeTlsDomain {
    fn to_string(&self) -> String {
        match self {
            Self::TlsDomain => String::from("tls_domain"),
        }
    }
}

impl Default for TypeTlsDomain {
    fn default() -> TypeTlsDomain {
        Self::TlsDomain
    }
}




