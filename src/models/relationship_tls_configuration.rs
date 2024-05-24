/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// RelationshipTlsConfiguration : The [TLS configuration](https://www.fastly.com/documentation/reference/api/tls/custom-certs/configuration/) being used to terminate TLS traffic. Optional.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipTlsConfiguration {
    #[serde(rename = "tls_configuration", skip_serializing_if = "Option::is_none")]
    pub tls_configuration: Option<Box<crate::models::RelationshipTlsConfigurationTlsConfiguration>>,
}

impl RelationshipTlsConfiguration {
    /// The [TLS configuration](https://www.fastly.com/documentation/reference/api/tls/custom-certs/configuration/) being used to terminate TLS traffic. Optional.
    pub fn new() -> RelationshipTlsConfiguration {
        RelationshipTlsConfiguration {
            tls_configuration: None,
        }
    }
}


