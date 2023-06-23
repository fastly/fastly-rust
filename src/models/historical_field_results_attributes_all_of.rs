/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoricalFieldResultsAttributesAllOf {
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<String>>,
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i32>,
}

impl HistoricalFieldResultsAttributesAllOf {
    pub fn new() -> HistoricalFieldResultsAttributesAllOf {
        HistoricalFieldResultsAttributesAllOf {
            service_id: None,
            start_time: None,
        }
    }
}


