/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RuleResponseAllOf {
    /// Alphanumeric string identifying the rule. Stable across versions of the routing config.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether this is the default (catch-all) rule for the path.
    #[serde(rename = "is_default", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
    /// The conditions a request must satisfy for this rule to match. Empty for the default rule.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::RoutingConfigCondition>>,
}

impl RuleResponseAllOf {
    pub fn new() -> RuleResponseAllOf {
        RuleResponseAllOf {
            id: None,
            is_default: None,
            action: None,
            conditions: None,
        }
    }
}


