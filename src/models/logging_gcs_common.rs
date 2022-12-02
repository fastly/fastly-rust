/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingGcsCommon {
    /// Your Google Cloud Platform service account email address. The `client_email` field in your service account authentication JSON. Not required if `account_name` is specified.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Your Google Cloud Platform account secret key. The `private_key` field in your service account authentication JSON. Not required if `account_name` is specified.
    #[serde(rename = "secret_key", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// The name of the Google Cloud Platform service account associated with the target log collection service. Not required if `user` and `secret_key` are provided.
    #[serde(rename = "account_name", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}

impl LoggingGcsCommon {
    pub fn new() -> LoggingGcsCommon {
        LoggingGcsCommon {
            user: None,
            secret_key: None,
            account_name: None,
        }
    }
}


