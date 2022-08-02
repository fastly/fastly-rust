/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TlsPrivateKeyResponseAttributesAllOf {
    /// A customizable name for your private key.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The key length used to generate the private key.
    #[serde(rename = "key_length", skip_serializing_if = "Option::is_none")]
    pub key_length: Option<i32>,
    /// The algorithm used to generate the private key. Must be `RSA`.
    #[serde(rename = "key_type", skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    /// A recommendation from Fastly to replace this private key and all associated certificates.
    #[serde(rename = "replace", skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    /// Useful for safely identifying the key.
    #[serde(rename = "public_key_sha1", skip_serializing_if = "Option::is_none")]
    pub public_key_sha1: Option<String>,
}

impl TlsPrivateKeyResponseAttributesAllOf {
    pub fn new() -> TlsPrivateKeyResponseAttributesAllOf {
        TlsPrivateKeyResponseAttributesAllOf {
            name: None,
            key_length: None,
            key_type: None,
            replace: None,
            public_key_sha1: None,
        }
    }
}


