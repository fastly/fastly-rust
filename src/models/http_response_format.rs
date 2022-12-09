/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// HttpResponseFormat : Payload format for delivering to subscribers of whole HTTP responses (`response` hold mode). One of `body` or `body-bin` must be specified.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HttpResponseFormat {
    /// The HTTP status code.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// The HTTP status string. Defaults to a string appropriate for `code`.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// A map of arbitrary HTTP response headers to include in the response.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
    /// The response body as a string.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// The response body as a base64-encoded binary blob.
    #[serde(rename = "body-bin", skip_serializing_if = "Option::is_none")]
    pub body_bin: Option<String>,
}

impl HttpResponseFormat {
    /// Payload format for delivering to subscribers of whole HTTP responses (`response` hold mode). One of `body` or `body-bin` must be specified.
    pub fn new() -> HttpResponseFormat {
        HttpResponseFormat {
            code: None,
            reason: None,
            headers: None,
            body: None,
            body_bin: None,
        }
    }
}


