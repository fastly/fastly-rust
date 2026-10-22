/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotRequestMetricsResponse {
    /// An array of time series data items containing metric counts. Items are grouped by `timestamp` by default, or by a combination of `timestamp` and `bot` or `datacenter` or `region` when the `group_by` parameter is set. This dataset is sparse: only keys with non-zero values are included. 
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::BotRequestMetricsDataItem>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::BotRequestMetricsMetadata>>,
}

impl BotRequestMetricsResponse {
    pub fn new() -> BotRequestMetricsResponse {
        BotRequestMetricsResponse {
            data: None,
            meta: None,
        }
    }
}


