/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Notification {
    /// Notification type
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "config")]
    pub config: Box<crate::models::NotificationConfig>,
}

impl Notification {
    pub fn new(_type: Type, config: crate::models::NotificationConfig) -> Notification {
        Notification {
            _type,
            config: Box::new(config),
        }
    }
}

/// Notification type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "mailinglist")]
    Mailinglist,
}

impl Default for Type {
    fn default() -> Type {
        Self::Mailinglist
    }
}

