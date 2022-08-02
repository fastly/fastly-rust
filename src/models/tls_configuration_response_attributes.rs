/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TlsConfigurationResponseAttributes {
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Signifies whether or not Fastly will use this configuration as a default when creating a new [TLS Activation](/reference/api/tls/custom-certs/activations/).
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// HTTP protocols available on your configuration.
    #[serde(rename = "http_protocols", skip_serializing_if = "Option::is_none")]
    pub http_protocols: Option<Vec<String>>,
    /// TLS protocols available on your configuration.
    #[serde(rename = "tls_protocols", skip_serializing_if = "Option::is_none")]
    pub tls_protocols: Option<Vec<f32>>,
    /// Signifies whether the configuration is used for Platform TLS or not.
    #[serde(rename = "bulk", skip_serializing_if = "Option::is_none")]
    pub bulk: Option<bool>,
}

impl TlsConfigurationResponseAttributes {
    pub fn new() -> TlsConfigurationResponseAttributes {
        TlsConfigurationResponseAttributes {
            created_at: None,
            deleted_at: None,
            updated_at: None,
            default: None,
            http_protocols: None,
            tls_protocols: None,
            bulk: None,
        }
    }
}


