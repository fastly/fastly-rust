/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingKinesisAdditional {
    /// The name for the real-time logging configuration.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<crate::models::LoggingPlacement>,
    /// A Fastly [log format string](https://www.fastly.com/documentation/guides/integrations/streaming-logs/custom-log-formats/).
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The Amazon Kinesis stream to send logs to. Required.
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<crate::models::AwsRegion>,
    /// The secret key associated with the target Amazon Kinesis stream. Not required if `iam_role` is specified.
    #[serde(rename = "secret_key", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// The access key associated with the target Amazon Kinesis stream. Not required if `iam_role` is specified.
    #[serde(rename = "access_key", skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// The ARN for an IAM role granting Fastly access to the target Amazon Kinesis stream. Not required if `access_key` and `secret_key` are provided.
    #[serde(rename = "iam_role", skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
}

impl LoggingKinesisAdditional {
    pub fn new() -> LoggingKinesisAdditional {
        LoggingKinesisAdditional {
            name: None,
            placement: None,
            format: None,
            topic: None,
            region: None,
            secret_key: None,
            access_key: None,
            iam_role: None,
        }
    }
}


