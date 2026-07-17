/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ConditionOperator : The comparison operator used to evaluate the condition.

use std::fmt;

/// The comparison operator used to evaluate the condition.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConditionOperator {
    #[serde(rename = "equals")]
    ConditionOperatorEquals,
    #[serde(rename = "starts_with")]
    ConditionOperatorStartsWith,
    #[serde(rename = "ends_with")]
    ConditionOperatorEndsWith,
    #[serde(rename = "contains")]
    ConditionOperatorContains,

}

impl fmt::Display for ConditionOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConditionOperatorEquals => write!(f, "{}", "equals"),
            Self::ConditionOperatorStartsWith => write!(f, "{}", "starts_with"),
            Self::ConditionOperatorEndsWith => write!(f, "{}", "ends_with"),
            Self::ConditionOperatorContains => write!(f, "{}", "contains"),
        }
    }
}

impl Default for ConditionOperator {
    fn default() -> ConditionOperator {
        Self::ConditionOperatorEquals
    }
}




