/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// AgentKey : An agent key used for configuring a Next-Gen WAF agent.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AgentKey {
    /// Agent configuration access key value.
    #[serde(rename = "access_key")]
    pub access_key: String,
    /// Agent configuration secret key value.
    #[serde(rename = "secret_key")]
    pub secret_key: String,
    /// Whether the agent key is the primary key that should be used to configure the agent.
    #[serde(rename = "is_primary")]
    pub is_primary: bool,
    /// Date and time the agent key was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Date and time the agent key was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl AgentKey {
    /// An agent key used for configuring a Next-Gen WAF agent.
    pub fn new(access_key: String, secret_key: String, is_primary: bool, created_at: String, updated_at: String) -> AgentKey {
        AgentKey {
            access_key,
            secret_key,
            is_primary,
            created_at,
            updated_at,
        }
    }
}


