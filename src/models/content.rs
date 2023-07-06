/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "response_time", skip_serializing_if = "Option::is_none")]
    pub response_time: Option<f32>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(rename = "pop", skip_serializing_if = "Option::is_none")]
    pub pop: Option<String>,
}

impl Content {
    pub fn new() -> Content {
        Content {
            hash: None,
            request: None,
            response: None,
            response_time: None,
            server: None,
            pop: None,
        }
    }
}


