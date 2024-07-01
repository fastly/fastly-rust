/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// LoggingUseTlsString : Whether to use TLS.

/// Whether to use TLS.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoggingUseTlsString {
    #[serde(rename = "0")]
    NoTls,
    #[serde(rename = "1")]
    UseTls,

}

impl ToString for LoggingUseTlsString {
    fn to_string(&self) -> String {
        match self {
            Self::NoTls => String::from("0"),
            Self::UseTls => String::from("1"),
        }
    }
}

impl Default for LoggingUseTlsString {
    fn default() -> LoggingUseTlsString {
        Self::NoTls
    }
}




