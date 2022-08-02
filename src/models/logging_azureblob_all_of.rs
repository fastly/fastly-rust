/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingAzureblobAllOf {
    /// The path to upload logs to.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The unique Azure Blob Storage namespace in which your data objects are stored. Required.
    #[serde(rename = "account_name", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// The name of the Azure Blob Storage container in which to store logs. Required.
    #[serde(rename = "container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// The Azure shared access signature providing write access to the blob service objects. Be sure to update your token before it expires or the logging functionality will not work. Required.
    #[serde(rename = "sas_token", skip_serializing_if = "Option::is_none")]
    pub sas_token: Option<String>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    #[serde(rename = "public_key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// The maximum number of bytes for each uploaded file. A value of 0 can be used to indicate there is no limit on the size of uploaded files, otherwise the minimum value is 1048576 bytes (1 MiB.)
    #[serde(rename = "file_max_bytes", skip_serializing_if = "Option::is_none")]
    pub file_max_bytes: Option<i32>,
}

impl LoggingAzureblobAllOf {
    pub fn new() -> LoggingAzureblobAllOf {
        LoggingAzureblobAllOf {
            path: None,
            account_name: None,
            container: None,
            sas_token: None,
            public_key: None,
            file_max_bytes: None,
        }
    }
}


