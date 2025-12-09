/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// PlatformMetadata : Meta information about the scope of the query in a human readable format.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PlatformMetadata {
    /// An RFC-8339-formatted date and time indicating the inclusive start of the query time range.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// An RFC-8339-formatted date and time indicating the exclusive end of the query time range.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// A string that can be used to request the next page of results, if any.
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// A comma-separated list of fields used to group and order the results.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    /// The maximum number of results to return.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl PlatformMetadata {
    /// Meta information about the scope of the query in a human readable format.
    pub fn new() -> PlatformMetadata {
        PlatformMetadata {
            from: None,
            to: None,
            next_cursor: None,
            group_by: None,
            limit: None,
        }
    }
}


