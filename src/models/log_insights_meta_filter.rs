/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// LogInsightsMetaFilter : The filters that were supplied in the request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogInsightsMetaFilter {
    /// Specifies the ID of the service for which data should be returned.
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// Start time for the query as supplied in the request.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// End time for the query as supplied in the request.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Value of the `domain_exact_match` filter as supplied in the request.
    #[serde(rename = "domain_exact_match", skip_serializing_if = "Option::is_none")]
    pub domain_exact_match: Option<bool>,
    /// Number of records per page.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl LogInsightsMetaFilter {
    /// The filters that were supplied in the request.
    pub fn new() -> LogInsightsMetaFilter {
        LogInsightsMetaFilter {
            service_id: None,
            start: None,
            end: None,
            domain_exact_match: None,
            limit: None,
        }
    }
}


