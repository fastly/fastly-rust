/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectListMeta : Pagination metadata for a collection response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectListMeta {
    /// Effective page limit after defaulting and clamping.
    #[serde(rename = "limit")]
    pub limit: i32,
    /// Continuation value to send as cursor when more resources remain. Omitted on the final page.
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl NetworkProtectListMeta {
    /// Pagination metadata for a collection response.
    pub fn new(limit: i32) -> NetworkProtectListMeta {
        NetworkProtectListMeta {
            limit,
            next_cursor: None,
        }
    }
}


