/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaginationMeta {
    /// Current page.
    #[serde(rename = "current_page", skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// Number of records per page.
    #[serde(rename = "per_page", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    /// Total records in result set.
    #[serde(rename = "record_count", skip_serializing_if = "Option::is_none")]
    pub record_count: Option<i32>,
    /// Total pages in result set.
    #[serde(rename = "total_pages", skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i32>,
}

impl PaginationMeta {
    pub fn new() -> PaginationMeta {
        PaginationMeta {
            current_page: None,
            per_page: None,
            record_count: None,
            total_pages: None,
        }
    }
}


