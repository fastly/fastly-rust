/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CacheSetting {
    /// If set, will cause vcl_fetch to terminate after processing this rule with the return state specified. If not set, other configuration logic in vcl_fetch with a lower priority will run after this rule. 
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// Name of the cache condition controlling when this configuration applies.
    #[serde(rename = "cache_condition", skip_serializing_if = "Option::is_none")]
    pub cache_condition: Option<String>,
    /// Name for the cache settings object.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Maximum time in seconds to continue to use a stale version of the object if future requests to your backend server fail (also known as 'stale if error').
    #[serde(rename = "stale_ttl", skip_serializing_if = "Option::is_none")]
    pub stale_ttl: Option<i32>,
    /// Maximum time to consider the object fresh in the cache (the cache 'time to live').
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
}

impl CacheSetting {
    pub fn new() -> CacheSetting {
        CacheSetting {
            action: None,
            cache_condition: None,
            name: None,
            stale_ttl: None,
            ttl: None,
        }
    }
}

/// If set, will cause vcl_fetch to terminate after processing this rule with the return state specified. If not set, other configuration logic in vcl_fetch with a lower priority will run after this rule. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "cache")]
    Cache,
    #[serde(rename = "restart")]
    Restart,
}

impl Default for Action {
    fn default() -> Action {
        Self::Pass
    }
}

