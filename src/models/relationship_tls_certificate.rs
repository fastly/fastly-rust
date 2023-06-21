/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// RelationshipTlsCertificate : The [TLS certificate](/reference/api/tls/custom-certs/certificates/) being used to terminate TLS traffic for a domain. Required.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipTlsCertificate {
    #[serde(rename = "tls_certificate", skip_serializing_if = "Option::is_none")]
    pub tls_certificate: Option<Box<crate::models::RelationshipTlsCertificateTlsCertificate>>,
}

impl RelationshipTlsCertificate {
    /// The [TLS certificate](/reference/api/tls/custom-certs/certificates/) being used to terminate TLS traffic for a domain. Required.
    pub fn new() -> RelationshipTlsCertificate {
        RelationshipTlsCertificate {
            tls_certificate: None,
        }
    }
}


