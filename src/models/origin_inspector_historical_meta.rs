/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// OriginInspectorHistoricalMeta : Meta information about the scope of the query in a human readable format.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OriginInspectorHistoricalMeta {
    /// Start time that was used to perform the query as an ISO-8601-formatted date and time.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// End time that was used to perform the query as an ISO-8601-formatted date and time.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Downsample that was used to perform the query. One of `minute`, `hour` or `day`.
    #[serde(rename = "downsample", skip_serializing_if = "Option::is_none")]
    pub downsample: Option<String>,
    /// A comma-separated list of the metrics that were requested.
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<String>,
    /// The maximum number of results shown per page.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<f32>,
    /// A string that can be used to request the next page of results, if any.
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// A comma-separated list of keys the results are sorted by.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// A comma-separated list of dimensions being summed over in the query.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Box<crate::models::OriginInspectorHistoricalMetaFilters>>,
}

impl OriginInspectorHistoricalMeta {
    /// Meta information about the scope of the query in a human readable format.
    pub fn new() -> OriginInspectorHistoricalMeta {
        OriginInspectorHistoricalMeta {
            start: None,
            end: None,
            downsample: None,
            metrics: None,
            limit: None,
            next_cursor: None,
            sort: None,
            group_by: None,
            filters: None,
        }
    }
}


