/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// HistoricalDdosMeta : Meta information about the scope of the query in a human readable format.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoricalDdosMeta {
    /// Start time that was used to perform the query as an ISO-8601-formatted date and time.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// End time that was used to perform the query as an ISO-8601-formatted date and time.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Downsample that was used to perform the query. One of `hour` or `day`.
    #[serde(rename = "downsample", skip_serializing_if = "Option::is_none")]
    pub downsample: Option<String>,
    /// A comma-separated list of the metrics that were requested.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
}

impl HistoricalDdosMeta {
    /// Meta information about the scope of the query in a human readable format.
    pub fn new() -> HistoricalDdosMeta {
        HistoricalDdosMeta {
            start: None,
            end: None,
            downsample: None,
            metric: None,
        }
    }
}


