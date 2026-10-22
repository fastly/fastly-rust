/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcBedrockSetupResponse {
    /// The ARN of the AI Runtime Control IAM role your AWS account's trust policy must trust.
    #[serde(rename = "arc_role_arn")]
    pub arc_role_arn: String,
    /// The external ID to require in your trust policy. This is your Fastly customer ID.
    #[serde(rename = "external_id")]
    pub external_id: String,
}

impl ArcBedrockSetupResponse {
    pub fn new(arc_role_arn: String, external_id: String) -> ArcBedrockSetupResponse {
        ArcBedrockSetupResponse {
            arc_role_arn,
            external_id,
        }
    }
}


