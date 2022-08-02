/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TlsBulkCertificateDataAttributes {
    /// Allow certificates that chain to untrusted roots.
    #[serde(rename = "allow_untrusted_root", skip_serializing_if = "Option::is_none")]
    pub allow_untrusted_root: Option<bool>,
    /// The PEM-formatted certificate blob. Required.
    #[serde(rename = "cert_blob", skip_serializing_if = "Option::is_none")]
    pub cert_blob: Option<String>,
    /// The PEM-formatted chain of intermediate blobs. Required.
    #[serde(rename = "intermediates_blob", skip_serializing_if = "Option::is_none")]
    pub intermediates_blob: Option<String>,
}

impl TlsBulkCertificateDataAttributes {
    pub fn new() -> TlsBulkCertificateDataAttributes {
        TlsBulkCertificateDataAttributes {
            allow_untrusted_root: None,
            cert_blob: None,
            intermediates_blob: None,
        }
    }
}


