/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceListResponseAllOf {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<String>>,
    /// Current [version](https://www.fastly.com/documentation/reference/api/services/version/) of the service.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// A list of [versions](https://www.fastly.com/documentation/reference/api/services/version/) associated with the service.
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<crate::models::SchemasVersionResponse>>,
    /// A list of environments where the service has been deployed.
    #[serde(rename = "environments", skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<crate::models::Environment>>,
}

impl ServiceListResponseAllOf {
    pub fn new() -> ServiceListResponseAllOf {
        ServiceListResponseAllOf {
            id: None,
            version: None,
            versions: None,
            environments: None,
        }
    }
}


