/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Values503Responses {
    /// The HTTP request path.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The rate at which the reason in this dimension occurs among responses to this URL with a 503 status code.
    #[serde(rename = "rate_per_url", skip_serializing_if = "Option::is_none")]
    pub rate_per_url: Option<f32>,
    /// The rate at which 503 status codes are returned for this URL.
    #[serde(rename = "503_rate_per_url", skip_serializing_if = "Option::is_none")]
    pub var_503_rate_per_url: Option<f32>,
}

impl Values503Responses {
    pub fn new() -> Values503Responses {
        Values503Responses {
            url: None,
            rate_per_url: None,
            var_503_rate_per_url: None,
        }
    }
}


