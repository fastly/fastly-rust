/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaginationLinks {
    /// The first page of data.
    #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    /// The last page of data.
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    /// The previous page of data.
    #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    /// The next page of data.
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

impl PaginationLinks {
    pub fn new() -> PaginationLinks {
        PaginationLinks {
            first: None,
            last: None,
            prev: None,
            next: None,
        }
    }
}


