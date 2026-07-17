/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// RuleUpdate : All attributes for updating a rule. At least one of `action`, `conditions`, or `position` must be provided.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RuleUpdate {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::RoutingConfigCondition>>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<crate::models::Position>>,
}

impl RuleUpdate {
    /// All attributes for updating a rule. At least one of `action`, `conditions`, or `position` must be provided.
    pub fn new() -> RuleUpdate {
        RuleUpdate {
            action: None,
            conditions: None,
            position: None,
        }
    }
}


