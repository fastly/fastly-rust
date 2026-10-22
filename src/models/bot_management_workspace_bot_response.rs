/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotManagementWorkspaceBotResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::BotManagementBotAction>,
    /// The unique identifier of the bot.
    #[serde(rename = "bot_id")]
    pub bot_id: String,
    /// The display name of the bot.
    #[serde(rename = "name")]
    pub name: String,
    /// The ID of the category the bot belongs to.
    #[serde(rename = "category_id")]
    pub category_id: String,
}

impl BotManagementWorkspaceBotResponse {
    pub fn new(action: crate::models::BotManagementBotAction, bot_id: String, name: String, category_id: String) -> BotManagementWorkspaceBotResponse {
        BotManagementWorkspaceBotResponse {
            action: Box::new(action),
            bot_id,
            name,
            category_id,
        }
    }
}


