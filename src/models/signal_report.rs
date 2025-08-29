/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignalReport {
    /// Name of the attack type.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Display name of the attack type.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Total count of attacks of this type.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// Top workspaces affected by this attack type.
    #[serde(rename = "top_workspaces", skip_serializing_if = "Option::is_none")]
    pub top_workspaces: Option<Vec<crate::models::TopWorkspace>>,
}

impl SignalReport {
    pub fn new() -> SignalReport {
        SignalReport {
            name: None,
            display_name: None,
            count: None,
            top_workspaces: None,
        }
    }
}


