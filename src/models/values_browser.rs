/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValuesBrowser {
    /// The version of the client's browser.
    #[serde(rename = "browser_version", skip_serializing_if = "Option::is_none")]
    pub browser_version: Option<String>,
    /// The percentage of requests by this version of the browser specified in the dimension.
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
}

impl ValuesBrowser {
    pub fn new() -> ValuesBrowser {
        ValuesBrowser {
            browser_version: None,
            rate: None,
        }
    }
}


