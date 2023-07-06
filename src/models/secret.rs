/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Secret {
    /// A human-readable name for the secret. The value must contain only letters, numbers, dashes (`-`), underscores (`_`), and periods (`.`).
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A Base64-encoded string containing either the secret or the encrypted secret (when using client_key). The maximum secret size (before Base64 encoding and optional local encryption) is 64KB.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The Base64-encoded string containing the client key used to encrypt the secret, if applicable.
    #[serde(rename = "client_key", skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,
}

impl Secret {
    pub fn new() -> Secret {
        Secret {
            name: None,
            secret: None,
            client_key: None,
        }
    }
}


