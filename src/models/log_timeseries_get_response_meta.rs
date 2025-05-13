/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogTimeseriesGetResponseMeta {
    /// ID of the service for which data was returned.
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// Start time for the query as supplied in the request.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// End time for the query as supplied in the request.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// The granularity of the time buckets in the response.
    #[serde(rename = "granularity", skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Box<crate::models::LogTimeseriesGetResponseMetaFilters>>,
}

impl LogTimeseriesGetResponseMeta {
    pub fn new() -> LogTimeseriesGetResponseMeta {
        LogTimeseriesGetResponseMeta {
            service_id: None,
            start: None,
            end: None,
            granularity: None,
            filters: None,
        }
    }
}


