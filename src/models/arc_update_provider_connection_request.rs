/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcUpdateProviderConnectionRequest : A partial update to a provider connection. Only the fields you include are changed. `models`, `base_url`, and `api_key` apply to `api-key` connections; `customer_role_arn` and `region` apply to `aws-iam` connections. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcUpdateProviderConnectionRequest {
    /// The AI model identifiers this connection is allowed to use. Must be non-empty if included.
    #[serde(rename = "models", skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,
    /// The base URL of the provider's API (`api-key` connections only).
    #[serde(rename = "base_url", skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// The provider's secret API key (`api-key` connections only).
    #[serde(rename = "api_key", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// The ARN of the IAM role to assume for Bedrock requests (`aws-iam` connections only).
    #[serde(rename = "customer_role_arn", skip_serializing_if = "Option::is_none")]
    pub customer_role_arn: Option<String>,
    /// The AWS region for Bedrock API requests (`aws-iam` connections only).
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl ArcUpdateProviderConnectionRequest {
    /// A partial update to a provider connection. Only the fields you include are changed. `models`, `base_url`, and `api_key` apply to `api-key` connections; `customer_role_arn` and `region` apply to `aws-iam` connections. 
    pub fn new() -> ArcUpdateProviderConnectionRequest {
        ArcUpdateProviderConnectionRequest {
            models: None,
            base_url: None,
            api_key: None,
            customer_role_arn: None,
            region: None,
        }
    }
}


