/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipsForTlsBulkCertificate {
    #[serde(rename = "tls_configurations", skip_serializing_if = "Option::is_none")]
    pub tls_configurations: Option<Box<crate::models::RelationshipTlsConfigurationsTlsConfigurations>>,
    #[serde(rename = "tls_domains", skip_serializing_if = "Option::is_none")]
    pub tls_domains: Option<Box<crate::models::RelationshipTlsDomainsTlsDomains>>,
}

impl RelationshipsForTlsBulkCertificate {
    pub fn new() -> RelationshipsForTlsBulkCertificate {
        RelationshipsForTlsBulkCertificate {
            tls_configurations: None,
            tls_domains: None,
        }
    }
}


