/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncludedWithTlsConfigurationItem {
    /// The IP address or hostname of the DNS record.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::TypeTlsDnsRecord>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::TlsDnsRecord>>,
}

impl IncludedWithTlsConfigurationItem {
    pub fn new() -> IncludedWithTlsConfigurationItem {
        IncludedWithTlsConfigurationItem {
            id: None,
            _type: None,
            attributes: None,
        }
    }
}


