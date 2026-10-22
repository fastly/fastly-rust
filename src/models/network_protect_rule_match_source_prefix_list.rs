/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRuleMatchSourcePrefixList : Requires a source prefix-list match field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectRuleMatchSourcePrefixList {
    /// Match traffic whose source appears in this prefix list.
    #[serde(rename = "source_prefix_list_id")]
    pub source_prefix_list_id: String,
}

impl NetworkProtectRuleMatchSourcePrefixList {
    /// Requires a source prefix-list match field.
    pub fn new(source_prefix_list_id: String) -> NetworkProtectRuleMatchSourcePrefixList {
        NetworkProtectRuleMatchSourcePrefixList {
            source_prefix_list_id,
        }
    }
}


