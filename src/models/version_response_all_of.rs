/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VersionResponseAllOf {
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<String>>,
    /// A list of environments where the service has been deployed.
    #[serde(rename = "environments", skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<crate::models::Environment>>,
}

impl VersionResponseAllOf {
    pub fn new() -> VersionResponseAllOf {
        VersionResponseAllOf {
            service_id: None,
            environments: None,
        }
    }
}


