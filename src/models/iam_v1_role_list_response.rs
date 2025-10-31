/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// IamV1RoleListResponse : Paginated list of IAM roles.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IamV1RoleListResponse {
    /// Maximum number of results returned.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Cursor for the next page.
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Page of IAM roles (length ≤ limit).
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::IamV1RoleResponse>>,
}

impl IamV1RoleListResponse {
    /// Paginated list of IAM roles.
    pub fn new() -> IamV1RoleListResponse {
        IamV1RoleListResponse {
            limit: None,
            next_cursor: None,
            data: None,
        }
    }
}


