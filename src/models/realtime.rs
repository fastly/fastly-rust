/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Realtime {
    /// Value to use for subsequent requests.
    #[serde(rename = "Timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Box<i32>>,
    /// How long the system will wait before aggregating messages for each second. The most recent data returned will have happened at the moment of the request, minus the aggregation delay.
    #[serde(rename = "AggregateDelay", skip_serializing_if = "Option::is_none")]
    pub aggregate_delay: Option<i32>,
    /// A list of [records](#record-data-model), each representing one second of time.
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::RealtimeEntry>>,
}

impl Realtime {
    pub fn new() -> Realtime {
        Realtime {
            timestamp: None,
            aggregate_delay: None,
            data: None,
        }
    }
}


