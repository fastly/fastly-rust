/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Settings {
    /// The default host name for the version.
    #[serde(rename = "general.default_host", skip_serializing_if = "Option::is_none")]
    pub general_default_host: Option<String>,
    /// The default time-to-live (TTL) for the version.
    #[serde(rename = "general.default_ttl", skip_serializing_if = "Option::is_none")]
    pub general_default_ttl: Option<i32>,
    /// Enables serving a stale object if there is an error.
    #[serde(rename = "general.stale_if_error", skip_serializing_if = "Option::is_none")]
    pub general_stale_if_error: Option<bool>,
    /// The default time-to-live (TTL) for serving the stale object for the version.
    #[serde(rename = "general.stale_if_error_ttl", skip_serializing_if = "Option::is_none")]
    pub general_stale_if_error_ttl: Option<i32>,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            general_default_host: None,
            general_default_ttl: None,
            general_stale_if_error: None,
            general_stale_if_error_ttl: None,
        }
    }
}


