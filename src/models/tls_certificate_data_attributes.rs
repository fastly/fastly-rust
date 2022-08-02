/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TlsCertificateDataAttributes {
    /// The PEM-formatted certificate blob. Required.
    #[serde(rename = "cert_blob", skip_serializing_if = "Option::is_none")]
    pub cert_blob: Option<String>,
    /// A customizable name for your certificate. Defaults to the certificate's Common Name or first Subject Alternative Names (SAN) entry. Optional.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl TlsCertificateDataAttributes {
    pub fn new() -> TlsCertificateDataAttributes {
        TlsCertificateDataAttributes {
            cert_blob: None,
            name: None,
        }
    }
}


