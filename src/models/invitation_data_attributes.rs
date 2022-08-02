/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvitationDataAttributes {
    /// The email address of the invitee.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Indicates the user has limited access to the customer's services.
    #[serde(rename = "limit_services", skip_serializing_if = "Option::is_none")]
    pub limit_services: Option<bool>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::models::RoleUser>,
    /// Indicates whether or not the invitation is active.
    #[serde(rename = "status_code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<StatusCode>,
}

impl InvitationDataAttributes {
    pub fn new() -> InvitationDataAttributes {
        InvitationDataAttributes {
            email: None,
            limit_services: None,
            role: None,
            status_code: None,
        }
    }
}

/// Indicates whether or not the invitation is active.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCode {
    #[serde(rename = "0")]
    StatusCodeInactive,
    #[serde(rename = "1")]
    StatusCodeActive,
}

impl Default for StatusCode {
    fn default() -> StatusCode {
        Self::StatusCodeInactive
    }
}

