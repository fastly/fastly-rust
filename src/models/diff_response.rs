/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DiffResponse {
    /// The version number being diffed from.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,
    /// The version number being diffed to.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<i32>,
    /// The format the diff is being returned in (`text`, `html` or `html_simple`).
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The differences between two specified service versions. Returns the full config if the version configurations are identical.
    #[serde(rename = "diff", skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
}

impl DiffResponse {
    pub fn new() -> DiffResponse {
        DiffResponse {
            from: None,
            to: None,
            format: None,
            diff: None,
        }
    }
}


