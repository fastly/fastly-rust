/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcRotateVirtualKeyRequest : Options for rotating a virtual key.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcRotateVirtualKeyRequest {
    /// The date and time the new access token expires, in RFC 3339 format.
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl ArcRotateVirtualKeyRequest {
    /// Options for rotating a virtual key.
    pub fn new() -> ArcRotateVirtualKeyRequest {
        ArcRotateVirtualKeyRequest {
            expires_at: None,
        }
    }
}


