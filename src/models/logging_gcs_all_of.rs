/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingGcsAllOf {
    /// The name of the GCS bucket.
    #[serde(rename = "bucket_name", skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// The path to upload logs to.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<Box<String>>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    #[serde(rename = "public_key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// Your Google Cloud Platform project ID. Required
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl LoggingGcsAllOf {
    pub fn new() -> LoggingGcsAllOf {
        LoggingGcsAllOf {
            bucket_name: None,
            path: None,
            public_key: None,
            project_id: None,
        }
    }
}


