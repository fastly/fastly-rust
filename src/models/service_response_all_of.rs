/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceResponseAllOf {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<String>>,
    /// Unused at this time.
    #[serde(rename = "publish_key", skip_serializing_if = "Option::is_none")]
    pub publish_key: Option<String>,
    /// Whether the service is paused. Services are paused due to a lack of traffic for an extended period of time. Services are resumed either when a draft version is activated or a locked version is cloned and reactivated.
    #[serde(rename = "paused", skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// A list of [versions](/reference/api/services/version/) associated with the service.
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<crate::models::SchemasVersionResponse>>,
}

impl ServiceResponseAllOf {
    pub fn new() -> ServiceResponseAllOf {
        ServiceResponseAllOf {
            id: None,
            publish_key: None,
            paused: None,
            versions: None,
        }
    }
}


