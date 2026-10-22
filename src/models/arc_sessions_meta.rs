/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcSessionsMeta : Metadata about the request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcSessionsMeta {
    /// The `limit` value used when making the request.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// The `sort` value used when making the request.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// The count of results matching the request.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// The cursor to use to request the next page of results. `null` when there are no more results.
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// The cursor to use to request the previous page of results. `null` when there is no previous page.
    #[serde(rename = "prev_cursor", skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
}

impl ArcSessionsMeta {
    /// Metadata about the request.
    pub fn new() -> ArcSessionsMeta {
        ArcSessionsMeta {
            limit: None,
            sort: None,
            total: None,
            next_cursor: None,
            prev_cursor: None,
        }
    }
}


