/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Condition {
    /// A freeform descriptive note.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Name of the condition. Required.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A numeric string. Priority determines execution order. Lower numbers execute first.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// A conditional expression in VCL used to determine if the condition is met.
    #[serde(rename = "statement", skip_serializing_if = "Option::is_none")]
    pub statement: Option<String>,
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<String>>,
    /// A numeric string that represents the service version.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Type of the condition. Required.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
}

impl Condition {
    pub fn new() -> Condition {
        Condition {
            comment: None,
            name: None,
            priority: None,
            statement: None,
            service_id: None,
            version: None,
            _type: None,
        }
    }
}

/// Type of the condition. Required.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "REQUEST")]
    REQUEST,
    #[serde(rename = "CACHE")]
    CACHE,
    #[serde(rename = "RESPONSE")]
    RESPONSE,
    #[serde(rename = "PREFETCH")]
    PREFETCH,
}

impl Default for Type {
    fn default() -> Type {
        Self::REQUEST
    }
}

