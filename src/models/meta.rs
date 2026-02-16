/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// Meta : Metadata about the request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Meta {
    /// The `limit` value used when making the request.
    #[serde(rename = "limit")]
    pub limit: i32,
    /// The count of requests matching the filter.
    #[serde(rename = "total")]
    pub total: i32,
}

impl Meta {
    /// Metadata about the request.
    pub fn new(limit: i32, total: i32) -> Meta {
        Meta {
            limit,
            total,
        }
    }
}


