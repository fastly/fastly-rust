/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// ClientKey : A Base64-encoded X25519 public key.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClientKey {
    /// A Base64-encoded X25519 public key that can be used with a [libsodium-compatible sealed box](https://libsodium.gitbook.io/doc/public-key_cryptography/sealed_boxes) to encrypt secrets before upload.
    #[serde(rename = "client_key", skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,
    /// A Base64-encoded signature of the client key. The signature is generated using the signing key and must be verified before using the client key.
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl ClientKey {
    /// A Base64-encoded X25519 public key.
    pub fn new() -> ClientKey {
        ClientKey {
            client_key: None,
            signature: None,
            expires_at: None,
        }
    }
}


