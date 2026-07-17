/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// RuleCreate : All attributes for creating a rule. A rule with an empty `conditions` array is the default (catch-all) rule for its path.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RuleCreate {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
    /// The conditions a request must satisfy for this rule to match. An empty array indicates the default rule for the path.
    #[serde(rename = "conditions")]
    pub conditions: Vec<crate::models::RoutingConfigCondition>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<crate::models::Position>>,
}

impl RuleCreate {
    /// All attributes for creating a rule. A rule with an empty `conditions` array is the default (catch-all) rule for its path.
    pub fn new(action: crate::models::Action, conditions: Vec<crate::models::RoutingConfigCondition>) -> RuleCreate {
        RuleCreate {
            action: Box::new(action),
            conditions,
            position: None,
        }
    }
}


