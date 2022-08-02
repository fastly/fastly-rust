/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponseObject {
    /// Name of the cache condition controlling when this configuration applies.
    #[serde(rename = "cache_condition", skip_serializing_if = "Option::is_none")]
    pub cache_condition: Option<String>,
    /// The content to deliver for the response object, can be empty.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The MIME type of the content, can be empty.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Name for the request settings.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The HTTP status code.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// The HTTP response.
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// Condition which, if met, will select this configuration during a request. Optional.
    #[serde(rename = "request_condition", skip_serializing_if = "Option::is_none")]
    pub request_condition: Option<String>,
}

impl ResponseObject {
    pub fn new() -> ResponseObject {
        ResponseObject {
            cache_condition: None,
            content: None,
            content_type: None,
            name: None,
            status: None,
            response: None,
            request_condition: None,
        }
    }
}


