/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Gzip {
    /// Name of the cache condition controlling when this configuration applies.
    #[serde(rename = "cache_condition", skip_serializing_if = "Option::is_none")]
    pub cache_condition: Option<String>,
    /// Space-separated list of content types to compress. If you omit this field a default list will be used.
    #[serde(rename = "content_types", skip_serializing_if = "Option::is_none")]
    pub content_types: Option<String>,
    /// Space-separated list of file extensions to compress. If you omit this field a default list will be used.
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<String>,
    /// Name of the gzip configuration.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Gzip {
    pub fn new() -> Gzip {
        Gzip {
            cache_condition: None,
            content_types: None,
            extensions: None,
            name: None,
        }
    }
}


