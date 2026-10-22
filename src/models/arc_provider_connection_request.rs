/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcProviderConnectionRequest {
    /// The provider name. Must be `bedrock` for `aws-iam` connections.
    #[serde(rename = "name")]
    pub name: Name,
    /// The Bedrock model identifiers this connection is allowed to use.
    #[serde(rename = "models")]
    pub models: Vec<String>,
    /// The base URL of the provider's API. Must be an absolute `http` or `https` URL and must not end in `/v1`.
    #[serde(rename = "base_url")]
    pub base_url: String,
    /// The provider's secret API key.
    #[serde(rename = "api_key")]
    pub api_key: String,
    /// The authentication type for this connection.
    #[serde(rename = "auth_type")]
    pub auth_type: AuthType,
    /// The ARN of the IAM role in your AWS account that AI Runtime Control assumes to access Bedrock. See [get Bedrock setup information](https://www.fastly.com/documentation/reference/api/ai-runtime-control/providers#arc-get-bedrock-setup) for the role ARN and external ID your trust policy must reference.
    #[serde(rename = "customer_role_arn")]
    pub customer_role_arn: String,
    /// The AWS region for Bedrock API requests.
    #[serde(rename = "region")]
    pub region: String,
}

impl ArcProviderConnectionRequest {
    pub fn new(name: Name, models: Vec<String>, base_url: String, api_key: String, auth_type: AuthType, customer_role_arn: String, region: String) -> ArcProviderConnectionRequest {
        ArcProviderConnectionRequest {
            name,
            models,
            base_url,
            api_key,
            auth_type,
            customer_role_arn,
            region,
        }
    }
}

/// The provider name. Must be `bedrock` for `aws-iam` connections.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "bedrock")]
    Bedrock,
}

impl Default for Name {
    fn default() -> Name {
        Self::Bedrock
    }
}
/// The authentication type for this connection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthType {
    #[serde(rename = "aws-iam")]
    AwsIam,
}

impl Default for AuthType {
    fn default() -> AuthType {
        Self::AwsIam
    }
}

