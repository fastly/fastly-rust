/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingCloudfilesAllOf {
    /// Your Cloud Files account access key.
    #[serde(rename = "access_key", skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// The name of your Cloud Files container.
    #[serde(rename = "bucket_name", skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// The path to upload logs to.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The region to stream logs to.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<Region>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    #[serde(rename = "public_key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// The username for your Cloud Files account.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl LoggingCloudfilesAllOf {
    pub fn new() -> LoggingCloudfilesAllOf {
        LoggingCloudfilesAllOf {
            access_key: None,
            bucket_name: None,
            path: None,
            region: None,
            public_key: None,
            user: None,
        }
    }
}

/// The region to stream logs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "DFW")]
    DFW,
    #[serde(rename = "ORD")]
    ORD,
    #[serde(rename = "IAD")]
    IAD,
    #[serde(rename = "LON")]
    LON,
    #[serde(rename = "SYD")]
    SYD,
    #[serde(rename = "HKG")]
    HKG,
    #[serde(rename = "null")]
    Null,
}

impl Default for Region {
    fn default() -> Region {
        Self::DFW
    }
}

