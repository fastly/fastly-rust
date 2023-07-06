/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// SigningKey : Used to verify signatures of client keys.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SigningKey {
    /// A Base64-encoded Ed25519 public key that can be used to verify signatures of client keys.
    #[serde(rename = "signing_key", skip_serializing_if = "Option::is_none")]
    pub signing_key: Option<String>,
}

impl SigningKey {
    /// Used to verify signatures of client keys.
    pub fn new() -> SigningKey {
        SigningKey {
            signing_key: None,
        }
    }
}


