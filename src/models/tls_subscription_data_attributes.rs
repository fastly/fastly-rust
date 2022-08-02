/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TlsSubscriptionDataAttributes {
    /// The entity that issues and certifies the TLS certificates for your subscription.
    #[serde(rename = "certificate_authority", skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<CertificateAuthority>,
}

impl TlsSubscriptionDataAttributes {
    pub fn new() -> TlsSubscriptionDataAttributes {
        TlsSubscriptionDataAttributes {
            certificate_authority: None,
        }
    }
}

/// The entity that issues and certifies the TLS certificates for your subscription.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CertificateAuthority {
    #[serde(rename = "lets-encrypt")]
    LetsEncrypt,
    #[serde(rename = "globalsign")]
    Globalsign,
}

impl Default for CertificateAuthority {
    fn default() -> CertificateAuthority {
        Self::LetsEncrypt
    }
}

