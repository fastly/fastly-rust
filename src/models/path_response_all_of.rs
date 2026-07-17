/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PathResponseAllOf {
    /// Alphanumeric string identifying the path. Stable across versions of the routing config.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URL path pattern, beginning with `/`. Maximum 2048 characters.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// HATEOAS links to related resources.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<::std::collections::HashMap<String, String>>,
}

impl PathResponseAllOf {
    pub fn new() -> PathResponseAllOf {
        PathResponseAllOf {
            id: None,
            path: None,
            links: None,
        }
    }
}


