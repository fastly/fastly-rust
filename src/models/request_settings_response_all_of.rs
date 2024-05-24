/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestSettingsResponseAllOf {
    /// Disable collapsed forwarding, so you don't wait for other objects to origin.
    #[serde(rename = "bypass_busy_wait", skip_serializing_if = "Option::is_none")]
    pub bypass_busy_wait: Option<String>,
    /// Allows you to force a cache miss for the request. Replaces the item in the cache if the content is cacheable.
    #[serde(rename = "force_miss", skip_serializing_if = "Option::is_none")]
    pub force_miss: Option<String>,
    /// Forces the request use SSL (redirects a non-SSL to SSL).
    #[serde(rename = "force_ssl", skip_serializing_if = "Option::is_none")]
    pub force_ssl: Option<String>,
    /// Injects Fastly-Geo-Country, Fastly-Geo-City, and Fastly-Geo-Region into the request headers.
    #[serde(rename = "geo_headers", skip_serializing_if = "Option::is_none")]
    pub geo_headers: Option<String>,
    /// How old an object is allowed to be to serve stale-if-error or stale-while-revalidate.
    #[serde(rename = "max_stale_age", skip_serializing_if = "Option::is_none")]
    pub max_stale_age: Option<String>,
    /// Injects the X-Timer info into the request for viewing origin fetch durations.
    #[serde(rename = "timer_support", skip_serializing_if = "Option::is_none")]
    pub timer_support: Option<String>,
}

impl RequestSettingsResponseAllOf {
    pub fn new() -> RequestSettingsResponseAllOf {
        RequestSettingsResponseAllOf {
            bypass_busy_wait: None,
            force_miss: None,
            force_ssl: None,
            geo_headers: None,
            max_stale_age: None,
            timer_support: None,
        }
    }
}


