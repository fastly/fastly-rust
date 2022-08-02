/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// LoggingMessageType : How the message should be formatted.

/// How the message should be formatted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoggingMessageType {
    #[serde(rename = "classic")]
    Classic,
    #[serde(rename = "loggly")]
    Loggly,
    #[serde(rename = "logplex")]
    Logplex,
    #[serde(rename = "blank")]
    Blank,

}

impl ToString for LoggingMessageType {
    fn to_string(&self) -> String {
        match self {
            Self::Classic => String::from("classic"),
            Self::Loggly => String::from("loggly"),
            Self::Logplex => String::from("logplex"),
            Self::Blank => String::from("blank"),
        }
    }
}

impl Default for LoggingMessageType {
    fn default() -> LoggingMessageType {
        Self::Classic
    }
}




