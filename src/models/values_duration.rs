/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValuesDuration {
    /// The average time in seconds to respond to requests to the URL in the current dimension.
    #[serde(rename = "average_response_time", skip_serializing_if = "Option::is_none")]
    pub average_response_time: Option<f32>,
    /// The P95 time in seconds to respond to requests to the URL in the current dimension.
    #[serde(rename = "p95_response_time", skip_serializing_if = "Option::is_none")]
    pub p95_response_time: Option<f32>,
    /// The total percentage of time to respond to all requests to the URL in the current dimension.
    #[serde(rename = "response_time_percentage", skip_serializing_if = "Option::is_none")]
    pub response_time_percentage: Option<f32>,
}

impl ValuesDuration {
    pub fn new() -> ValuesDuration {
        ValuesDuration {
            average_response_time: None,
            p95_response_time: None,
            response_time_percentage: None,
        }
    }
}


