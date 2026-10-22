/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcProviderConnectionResponse : A provider connection.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcProviderConnectionResponse {
    /// The unique identifier of the provider connection.
    #[serde(rename = "id")]
    pub id: String,
    /// An opaque identifier for the secret material backing this connection. Not the secret itself.
    #[serde(rename = "secret_id")]
    pub secret_id: String,
    /// The display name of the provider connection.
    #[serde(rename = "name")]
    pub name: String,
    /// The AI model identifiers this connection is allowed to use.
    #[serde(rename = "models")]
    pub models: Vec<String>,
    /// The base URL of the provider's API. Present only for `api-key` connections.
    #[serde(rename = "base_url", skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// The authentication type for this connection.
    #[serde(rename = "auth_type")]
    pub auth_type: AuthType,
    /// The AWS region for Bedrock API requests. Present only for `aws-iam` connections.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The date and time the connection was created, in RFC 3339 format.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The date and time the connection was last updated, in RFC 3339 format.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ArcProviderConnectionResponse {
    /// A provider connection.
    pub fn new(id: String, secret_id: String, name: String, models: Vec<String>, auth_type: AuthType, created_at: String, updated_at: String) -> ArcProviderConnectionResponse {
        ArcProviderConnectionResponse {
            id,
            secret_id,
            name,
            models,
            base_url: None,
            auth_type,
            region: None,
            created_at,
            updated_at,
        }
    }
}

/// The authentication type for this connection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthType {
    #[serde(rename = "api-key")]
    ApiKey,
    #[serde(rename = "aws-iam")]
    AwsIam,
}

impl Default for AuthType {
    fn default() -> AuthType {
        Self::ApiKey
    }
}

