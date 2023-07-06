/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OriginInspector {
    #[serde(rename = "Timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Box<crate::models::OriginInspectorSubsequentRequestTimestamp>>,
    /// Offset of entry timestamps from the current time due to processing time.
    #[serde(rename = "AggregateDelay", skip_serializing_if = "Option::is_none")]
    pub aggregate_delay: Option<i32>,
    /// A list of report [entries](#entry-data-model), each representing one second of time.
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::OriginInspectorRealtimeEntry>>,
}

impl OriginInspector {
    pub fn new() -> OriginInspector {
        OriginInspector {
            timestamp: None,
            aggregate_delay: None,
            data: None,
        }
    }
}


