/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRuleSetResponseFields : Read-only fields and required fields in a rule-set response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectRuleSetResponseFields {
    /// A read-only unique identifier for a Network Protect resource.
    #[serde(rename = "id")]
    pub id: String,
    /// An ISO 8601 timestamp in UTC.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// An ISO 8601 timestamp in UTC.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl NetworkProtectRuleSetResponseFields {
    /// Read-only fields and required fields in a rule-set response.
    pub fn new(id: String, created_at: String, updated_at: String) -> NetworkProtectRuleSetResponseFields {
        NetworkProtectRuleSetResponseFields {
            id,
            created_at,
            updated_at,
        }
    }
}


