/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// RoutingConfigCondition : A condition that must be met for a rule to match.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RoutingConfigCondition {
    #[serde(rename = "type")]
    pub _type: crate::models::ConditionType,
    #[serde(rename = "operator")]
    pub operator: crate::models::ConditionOperator,
    /// The key to evaluate. For `header` conditions this is the header name. Required for `header` conditions.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The value to compare against using the operator.
    #[serde(rename = "value")]
    pub value: String,
}

impl RoutingConfigCondition {
    /// A condition that must be met for a rule to match.
    pub fn new(_type: crate::models::ConditionType, operator: crate::models::ConditionOperator, value: String) -> RoutingConfigCondition {
        RoutingConfigCondition {
            _type,
            operator,
            key: None,
            value,
        }
    }
}


