/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectConflict : Conflict returned when resource state prevents the request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectConflict {
    /// Short summary of the conflict.
    #[serde(rename = "title")]
    pub title: String,
    /// HTTP status code for this error.
    #[serde(rename = "status")]
    pub status: Status,
    /// Explanation of the conflicting resource state.
    #[serde(rename = "detail")]
    pub detail: String,
    /// Optional conflict error details.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::NetworkProtectConflictError>>,
}

impl NetworkProtectConflict {
    /// Conflict returned when resource state prevents the request.
    pub fn new(title: String, status: Status, detail: String) -> NetworkProtectConflict {
        NetworkProtectConflict {
            title,
            status,
            detail,
            errors: None,
        }
    }
}

/// HTTP status code for this error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "409")]
    Status409,
}

impl Default for Status {
    fn default() -> Status {
        Self::Status409
    }
}

