/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcProviderConnectionApiKey : A provider connection authenticated with the provider's own API key.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcProviderConnectionApiKey {
    /// A display name for the provider connection. Cannot match a name reserved by a built-in provider.
    #[serde(rename = "name")]
    pub name: String,
    /// The AI model identifiers this connection is allowed to use.
    #[serde(rename = "models")]
    pub models: Vec<String>,
    /// The base URL of the provider's API. Must be an absolute `http` or `https` URL and must not end in `/v1`.
    #[serde(rename = "base_url")]
    pub base_url: String,
    /// The provider's secret API key.
    #[serde(rename = "api_key")]
    pub api_key: String,
    /// The authentication type for this connection. Omit for the default `api-key` behavior.
    #[serde(rename = "auth_type", skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AuthType>,
}

impl ArcProviderConnectionApiKey {
    /// A provider connection authenticated with the provider's own API key.
    pub fn new(name: String, models: Vec<String>, base_url: String, api_key: String) -> ArcProviderConnectionApiKey {
        ArcProviderConnectionApiKey {
            name,
            models,
            base_url,
            api_key,
            auth_type: None,
        }
    }
}

/// The authentication type for this connection. Omit for the default `api-key` behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthType {
    #[serde(rename = "api-key")]
    ApiKey,
}

impl Default for AuthType {
    fn default() -> AuthType {
        Self::ApiKey
    }
}

