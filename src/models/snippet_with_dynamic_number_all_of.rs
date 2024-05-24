/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SnippetWithDynamicNumberAllOf {
    /// Sets the snippet version.
    #[serde(rename = "dynamic", skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<Dynamic>,
}

impl SnippetWithDynamicNumberAllOf {
    pub fn new() -> SnippetWithDynamicNumberAllOf {
        SnippetWithDynamicNumberAllOf {
            dynamic: None,
        }
    }
}

/// Sets the snippet version.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Dynamic {
    #[serde(rename = "0")]
    Regular,
    #[serde(rename = "1")]
    Dynamic,
}

impl Default for Dynamic {
    fn default() -> Dynamic {
        Self::Regular
    }
}

