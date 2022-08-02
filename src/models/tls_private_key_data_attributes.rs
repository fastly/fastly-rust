/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TlsPrivateKeyDataAttributes {
    /// A customizable name for your private key. Optional.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The contents of the private key. Must be a PEM-formatted key. Not returned in response body. Required.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl TlsPrivateKeyDataAttributes {
    pub fn new() -> TlsPrivateKeyDataAttributes {
        TlsPrivateKeyDataAttributes {
            name: None,
            key: None,
        }
    }
}


