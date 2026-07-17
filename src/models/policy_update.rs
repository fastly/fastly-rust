/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PolicyUpdate {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(rename = "directives", skip_serializing_if = "Option::is_none")]
    pub directives: Option<Vec<crate::models::Directive>>,
}

impl PolicyUpdate {
    pub fn new() -> PolicyUpdate {
        PolicyUpdate {
            name: None,
            description: None,
            mode: None,
            directives: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "report")]
    Report,
    #[serde(rename = "enforce")]
    Enforce,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Report
    }
}

