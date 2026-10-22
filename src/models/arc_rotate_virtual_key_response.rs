/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcRotateVirtualKeyResponse : The virtual key that was rotated, including its new access token.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcRotateVirtualKeyResponse {
    /// The unique identifier of the virtual key.
    #[serde(rename = "id")]
    pub id: String,
    /// The ID of the customer account that owns the key.
    #[serde(rename = "customer_id")]
    pub customer_id: String,
    /// The display name of the virtual key.
    #[serde(rename = "name")]
    pub name: String,
    /// The ID of the user who created the key.
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// The date and time the key expires, in RFC 3339 format. `null` if the key doesn't expire.
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// The date and time the key was created, in RFC 3339 format.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The date and time the key was last updated, in RFC 3339 format.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The new access token for this virtual key. This is only returned when the key is created or rotated, and cannot be retrieved again afterward.
    #[serde(rename = "access_token")]
    pub access_token: String,
}

impl ArcRotateVirtualKeyResponse {
    /// The virtual key that was rotated, including its new access token.
    pub fn new(id: String, customer_id: String, name: String, user_id: String, created_at: String, updated_at: String, access_token: String) -> ArcRotateVirtualKeyResponse {
        ArcRotateVirtualKeyResponse {
            id,
            customer_id,
            name,
            user_id,
            expires_at: None,
            created_at,
            updated_at,
            access_token,
        }
    }
}


