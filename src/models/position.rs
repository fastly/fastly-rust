/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// Position : The position at which to place a new rule. At most one of `before` or `after` may be set. Cannot be used when creating a default (empty-conditions) rule.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Position {
    /// The id of the rule that this rule should be placed before.
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The id of the rule that this rule should be placed after.
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl Position {
    /// The position at which to place a new rule. At most one of `before` or `after` may be set. Cannot be used when creating a default (empty-conditions) rule.
    pub fn new() -> Position {
        Position {
            before: None,
            after: None,
        }
    }
}


