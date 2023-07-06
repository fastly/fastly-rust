/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecretResponse {
    /// Name of the secret.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An opaque identifier of the plaintext secret value. This can be used to determine if a secret value has changed.
    #[serde(rename = "digest", skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// True if the secret replaced a secret with the same name.
    #[serde(rename = "recreated", skip_serializing_if = "Option::is_none")]
    pub recreated: Option<bool>,
}

impl SecretResponse {
    pub fn new() -> SecretResponse {
        SecretResponse {
            name: None,
            digest: None,
            created_at: None,
            recreated: None,
        }
    }
}


