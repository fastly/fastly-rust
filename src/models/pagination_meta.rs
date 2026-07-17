/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// PaginationMeta : Cursor-based pagination metadata.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaginationMeta {
    /// The number of records returned per page.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Cursor value used to retrieve the next page of results. Empty if there are no more results.
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Cursor value used to retrieve the previous page of results. Empty if there is no previous page.
    #[serde(rename = "previous_cursor", skip_serializing_if = "Option::is_none")]
    pub previous_cursor: Option<String>,
    /// The sort order applied to the results.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

impl PaginationMeta {
    /// Cursor-based pagination metadata.
    pub fn new() -> PaginationMeta {
        PaginationMeta {
            limit: None,
            next_cursor: None,
            previous_cursor: None,
            sort: None,
        }
    }
}


