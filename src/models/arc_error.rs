/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcError : An error response. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcError {
    /// A URL identifying the error type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// A short summary of the problem.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A machine-readable error code.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl ArcError {
    /// An error response. 
    pub fn new() -> ArcError {
        ArcError {
            _type: None,
            title: None,
            code: None,
        }
    }
}


