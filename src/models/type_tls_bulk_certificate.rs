/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// TypeTlsBulkCertificate : Resource type

use std::fmt;

/// Resource type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeTlsBulkCertificate {
    #[serde(rename = "tls_bulk_certificate")]
    TlsBulkCertificate,

}

impl fmt::Display for TypeTlsBulkCertificate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TlsBulkCertificate => write!(f, "{}", "tls_bulk_certificate"),
        }
    }
}

impl Default for TypeTlsBulkCertificate {
    fn default() -> TypeTlsBulkCertificate {
        Self::TlsBulkCertificate
    }
}




