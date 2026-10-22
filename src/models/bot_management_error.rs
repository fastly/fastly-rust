/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// BotManagementError : An error response, formatted as a problem details object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotManagementError {
    /// A short summary of the problem.
    #[serde(rename = "title")]
    pub title: String,
    /// The HTTP status code of the response.
    #[serde(rename = "status")]
    pub status: i32,
    /// An explanation of the problem, specific to this occurrence.
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl BotManagementError {
    /// An error response, formatted as a problem details object.
    pub fn new(title: String, status: i32) -> BotManagementError {
        BotManagementError {
            title,
            status,
            detail: None,
        }
    }
}


