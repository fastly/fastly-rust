/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// BotManagementBotAction : The action applied to requests from a single bot. Set the action to 'inherit' to apply the category's action or to a specific value to override the category's action.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotManagementBotAction {
    /// The type of action to apply.
    #[serde(rename = "type")]
    pub _type: Type,
}

impl BotManagementBotAction {
    /// The action applied to requests from a single bot. Set the action to 'inherit' to apply the category's action or to a specific value to override the category's action.
    pub fn new(_type: Type) -> BotManagementBotAction {
        BotManagementBotAction {
            _type,
        }
    }
}

/// The type of action to apply.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "challenge")]
    Challenge,
    #[serde(rename = "inherit")]
    Inherit,
}

impl Default for Type {
    fn default() -> Type {
        Self::Allow
    }
}

