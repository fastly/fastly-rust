/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RoutingConfigResponseAllOf {
    /// Alphanumeric string identifying the routing config.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The user-defined name for the routing config.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::RoutingConfigState>,
    /// Timestamp of when the version was most recently activated. `null` if the version has never been activated.
    #[serde(rename = "activated_at", skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<String>,
    /// HATEOAS links to related resources.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<::std::collections::HashMap<String, String>>,
}

impl RoutingConfigResponseAllOf {
    pub fn new() -> RoutingConfigResponseAllOf {
        RoutingConfigResponseAllOf {
            id: None,
            name: None,
            state: None,
            activated_at: None,
            links: None,
        }
    }
}


