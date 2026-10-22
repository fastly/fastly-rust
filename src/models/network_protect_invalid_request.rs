/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectInvalidRequest : Validation error returned for an invalid request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectInvalidRequest {
    /// Short summary of the validation problem.
    #[serde(rename = "title")]
    pub title: String,
    /// HTTP status code for this error.
    #[serde(rename = "status")]
    pub status: Status,
    /// Corrective explanation for the invalid request.
    #[serde(rename = "detail")]
    pub detail: String,
    /// Field-level validation errors, when available.
    #[serde(rename = "errors")]
    pub errors: Vec<crate::models::NetworkProtectErrorDetail>,
}

impl NetworkProtectInvalidRequest {
    /// Validation error returned for an invalid request.
    pub fn new(title: String, status: Status, detail: String, errors: Vec<crate::models::NetworkProtectErrorDetail>) -> NetworkProtectInvalidRequest {
        NetworkProtectInvalidRequest {
            title,
            status,
            detail,
            errors,
        }
    }
}

/// HTTP status code for this error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "400")]
    Status400,
}

impl Default for Status {
    fn default() -> Status {
        Self::Status400
    }
}

