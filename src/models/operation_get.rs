/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OperationGet {
    /// The HTTP method for the operation.
    #[serde(rename = "method")]
    pub method: Method,
    /// The domain for the operation.
    #[serde(rename = "domain")]
    pub domain: String,
    /// The path for the operation, which may include path parameters.
    #[serde(rename = "path")]
    pub path: String,
    /// A description of what the operation does.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An array of operation tag IDs associated with this operation.
    #[serde(rename = "tag_ids", skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    /// The unique identifier of the operation.
    #[serde(rename = "id")]
    pub id: String,
    /// The timestamp when the operation was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The timestamp when the operation was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The timestamp when the operation was last seen in traffic.
    #[serde(rename = "last_seen_at", skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
}

impl OperationGet {
    pub fn new(method: Method, domain: String, path: String, id: String, updated_at: String) -> OperationGet {
        OperationGet {
            method,
            domain,
            path,
            description: None,
            tag_ids: None,
            id,
            created_at: None,
            updated_at,
            last_seen_at: None,
        }
    }
}

/// The HTTP method for the operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "OPTIONS")]
    OPTIONS,
    #[serde(rename = "CONNECT")]
    CONNECT,
    #[serde(rename = "TRACE")]
    TRACE,
}

impl Default for Method {
    fn default() -> Method {
        Self::GET
    }
}

