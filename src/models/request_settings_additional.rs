/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestSettingsAdditional {
    /// Allows you to terminate request handling and immediately perform an action.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// Sets the host header.
    #[serde(rename = "default_host", skip_serializing_if = "Option::is_none")]
    pub default_host: Option<String>,
    /// Comma separated list of varnish request object fields that should be in the hash key.
    #[serde(rename = "hash_keys", skip_serializing_if = "Option::is_none")]
    pub hash_keys: Option<String>,
    /// Name for the request settings.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Condition which, if met, will select this configuration during a request. Optional.
    #[serde(rename = "request_condition", skip_serializing_if = "Option::is_none")]
    pub request_condition: Option<String>,
    /// Short for X-Forwarded-For.
    #[serde(rename = "xff", skip_serializing_if = "Option::is_none")]
    pub xff: Option<Xff>,
}

impl RequestSettingsAdditional {
    pub fn new() -> RequestSettingsAdditional {
        RequestSettingsAdditional {
            action: None,
            default_host: None,
            hash_keys: None,
            name: None,
            request_condition: None,
            xff: None,
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

