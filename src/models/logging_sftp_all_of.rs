/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingSftpAllOf {
    /// The password for the server. If both `password` and `secret_key` are passed, `secret_key` will be used in preference.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// The path to upload logs to.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The port number.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    #[serde(rename = "public_key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// The SSH private key for the server. If both `password` and `secret_key` are passed, `secret_key` will be used in preference.
    #[serde(rename = "secret_key", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// A list of host keys for all hosts we can connect to over SFTP.
    #[serde(rename = "ssh_known_hosts", skip_serializing_if = "Option::is_none")]
    pub ssh_known_hosts: Option<String>,
    /// The username for the server.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl LoggingSftpAllOf {
    pub fn new() -> LoggingSftpAllOf {
        LoggingSftpAllOf {
            password: None,
            path: None,
            port: None,
            public_key: None,
            secret_key: None,
            ssh_known_hosts: None,
            user: None,
        }
    }
}


