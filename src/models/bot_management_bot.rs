/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// BotManagementBot : A single bot within a category. Bots are defined by Fastly and cannot be created or deleted.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotManagementBot {
    #[serde(rename = "action")]
    pub action: Box<crate::models::BotManagementBotAction>,
}

impl BotManagementBot {
    /// A single bot within a category. Bots are defined by Fastly and cannot be created or deleted.
    pub fn new(action: crate::models::BotManagementBotAction) -> BotManagementBot {
        BotManagementBot {
            action: Box::new(action),
        }
    }
}


