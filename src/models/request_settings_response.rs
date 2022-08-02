/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestSettingsResponse {
    /// Allows you to terminate request handling and immediately perform an action.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// Disable collapsed forwarding, so you don't wait for other objects to origin.
    #[serde(rename = "bypass_busy_wait", skip_serializing_if = "Option::is_none")]
    pub bypass_busy_wait: Option<i32>,
    /// Sets the host header.
    #[serde(rename = "default_host", skip_serializing_if = "Option::is_none")]
    pub default_host: Option<String>,
    /// Allows you to force a cache miss for the request. Replaces the item in the cache if the content is cacheable.
    #[serde(rename = "force_miss", skip_serializing_if = "Option::is_none")]
    pub force_miss: Option<i32>,
    /// Forces the request use SSL (redirects a non-SSL to SSL).
    #[serde(rename = "force_ssl", skip_serializing_if = "Option::is_none")]
    pub force_ssl: Option<i32>,
    /// Injects Fastly-Geo-Country, Fastly-Geo-City, and Fastly-Geo-Region into the request headers.
    #[serde(rename = "geo_headers", skip_serializing_if = "Option::is_none")]
    pub geo_headers: Option<i32>,
    /// Comma separated list of varnish request object fields that should be in the hash key.
    #[serde(rename = "hash_keys", skip_serializing_if = "Option::is_none")]
    pub hash_keys: Option<String>,
    /// How old an object is allowed to be to serve stale-if-error or stale-while-revalidate.
    #[serde(rename = "max_stale_age", skip_serializing_if = "Option::is_none")]
    pub max_stale_age: Option<i32>,
    /// Name for the request settings.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Condition which, if met, will select this configuration during a request. Optional.
    #[serde(rename = "request_condition", skip_serializing_if = "Option::is_none")]
    pub request_condition: Option<String>,
    /// Injects the X-Timer info into the request for viewing origin fetch durations.
    #[serde(rename = "timer_support", skip_serializing_if = "Option::is_none")]
    pub timer_support: Option<i32>,
    /// Short for X-Forwarded-For.
    #[serde(rename = "xff", skip_serializing_if = "Option::is_none")]
    pub xff: Option<Xff>,
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<String>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<i32>>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl RequestSettingsResponse {
    pub fn new() -> RequestSettingsResponse {
        RequestSettingsResponse {
            action: None,
            bypass_busy_wait: None,
            default_host: None,
            force_miss: None,
            force_ssl: None,
            geo_headers: None,
            hash_keys: None,
            max_stale_age: None,
            name: None,
            request_condition: None,
            timer_support: None,
            xff: None,
            service_id: None,
            version: None,
            created_at: None,
            deleted_at: None,
            updated_at: None,
        }
    }
}

/// Allows you to terminate request handling and immediately perform an action.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "lookup")]
    Lookup,
    #[serde(rename = "pass")]
    Pass,
}

impl Default for Action {
    fn default() -> Action {
        Self::Lookup
    }
}
/// Short for X-Forwarded-For.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Xff {
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "leave")]
    Leave,
    #[serde(rename = "append")]
    Append,
    #[serde(rename = "append_all")]
    AppendAll,
    #[serde(rename = "overwrite")]
    Overwrite,
}

impl Default for Xff {
    fn default() -> Xff {
        Self::Clear
    }
}

