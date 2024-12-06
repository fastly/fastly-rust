/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValuesBandwidth {
    /// The average bandwidth in bytes for responses to requests to the URL in the current dimension.
    #[serde(rename = "average_bandwidth_bytes", skip_serializing_if = "Option::is_none")]
    pub average_bandwidth_bytes: Option<f32>,
    /// The total bandwidth percentage for all responses to requests to the URL in the current dimension.
    #[serde(rename = "bandwidth_percentage", skip_serializing_if = "Option::is_none")]
    pub bandwidth_percentage: Option<f32>,
}

impl ValuesBandwidth {
    pub fn new() -> ValuesBandwidth {
        ValuesBandwidth {
            average_bandwidth_bytes: None,
            bandwidth_percentage: None,
        }
    }
}


