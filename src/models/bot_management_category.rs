/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// BotManagementCategory : A group of bots with similar characteristics or behaviors. Categories are defined by Fastly and cannot be created or deleted.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotManagementCategory {
    #[serde(rename = "action")]
    pub action: Box<crate::models::BotManagementAction>,
}

impl BotManagementCategory {
    /// A group of bots with similar characteristics or behaviors. Categories are defined by Fastly and cannot be created or deleted.
    pub fn new(action: crate::models::BotManagementAction) -> BotManagementCategory {
        BotManagementCategory {
            action: Box::new(action),
        }
    }
}


