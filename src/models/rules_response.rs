/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// RulesResponse : A paginated list of rules.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RulesResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::RuleResponse>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::PaginationMeta>>,
}

impl RulesResponse {
    /// A paginated list of rules.
    pub fn new() -> RulesResponse {
        RulesResponse {
            data: None,
            meta: None,
        }
    }
}


