/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRuleTimestamps : Read-only timestamps for an ordered rule. Moving a rule or changing a logical field or comment creates a new timestamp; appending a rule does not change timestamps for rules that remain in place.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectRuleTimestamps {
    /// An ISO 8601 timestamp in UTC.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// An ISO 8601 timestamp in UTC.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl NetworkProtectRuleTimestamps {
    /// Read-only timestamps for an ordered rule. Moving a rule or changing a logical field or comment creates a new timestamp; appending a rule does not change timestamps for rules that remain in place.
    pub fn new() -> NetworkProtectRuleTimestamps {
        NetworkProtectRuleTimestamps {
            created_at: None,
            updated_at: None,
        }
    }
}


