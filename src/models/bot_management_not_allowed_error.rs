/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotManagementNotAllowedError {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "status")]
    pub status: Status,
    /// An explanation of the problem, specific to this occurrence.
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl BotManagementNotAllowedError {
    pub fn new(title: String, status: Status) -> BotManagementNotAllowedError {
        BotManagementNotAllowedError {
            title,
            status,
            detail: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "403")]
    Status403,
}

impl Default for Status {
    fn default() -> Status {
        Self::Status403
    }
}

