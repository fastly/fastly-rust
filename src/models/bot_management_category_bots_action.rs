/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// BotManagementCategoryBotsAction : The action to apply to every bot in a category.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotManagementCategoryBotsAction {
    /// The type of action to apply. Only `inherit` is supported.
    #[serde(rename = "type")]
    pub _type: Type,
}

impl BotManagementCategoryBotsAction {
    /// The action to apply to every bot in a category.
    pub fn new(_type: Type) -> BotManagementCategoryBotsAction {
        BotManagementCategoryBotsAction {
            _type,
        }
    }
}

/// The type of action to apply. Only `inherit` is supported.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "inherit")]
    Inherit,
}

impl Default for Type {
    fn default() -> Type {
        Self::Inherit
    }
}

