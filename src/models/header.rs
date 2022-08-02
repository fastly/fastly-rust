/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Header {
    /// Accepts a string value.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// Name of the cache condition controlling when this configuration applies.
    #[serde(rename = "cache_condition", skip_serializing_if = "Option::is_none")]
    pub cache_condition: Option<String>,
    /// Header to set.
    #[serde(rename = "dst", skip_serializing_if = "Option::is_none")]
    pub dst: Option<String>,
    /// Don't add the header if it is added already. Only applies to 'set' action.
    #[serde(rename = "ignore_if_set", skip_serializing_if = "Option::is_none")]
    pub ignore_if_set: Option<i32>,
    /// A handle to refer to this Header object.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Priority determines execution order. Lower numbers execute first.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Regular expression to use. Only applies to `regex` and `regex_repeat` actions.
    #[serde(rename = "regex", skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    /// Condition which, if met, will select this configuration during a request. Optional.
    #[serde(rename = "request_condition", skip_serializing_if = "Option::is_none")]
    pub request_condition: Option<String>,
    /// Optional name of a response condition to apply.
    #[serde(rename = "response_condition", skip_serializing_if = "Option::is_none")]
    pub response_condition: Option<Box<String>>,
    /// Variable to be used as a source for the header content. Does not apply to `delete` action.
    #[serde(rename = "src", skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    /// Value to substitute in place of regular expression. Only applies to `regex` and `regex_repeat` actions.
    #[serde(rename = "substitution", skip_serializing_if = "Option::is_none")]
    pub substitution: Option<String>,
    /// Accepts a string value.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
}

impl Header {
    pub fn new() -> Header {
        Header {
            action: None,
            cache_condition: None,
            dst: None,
            ignore_if_set: None,
            name: None,
            priority: None,
            regex: None,
            request_condition: None,
            response_condition: None,
            src: None,
            substitution: None,
            _type: None,
        }
    }
}

/// Accepts a string value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "append")]
    Append,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "regex")]
    Regex,
    #[serde(rename = "regex_repeat")]
    RegexRepeat,
}

impl Default for Action {
    fn default() -> Action {
        Self::Set
    }
}
/// Accepts a string value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "cache")]
    Cache,
    #[serde(rename = "response")]
    Response,
}

impl Default for Type {
    fn default() -> Type {
        Self::Request
    }
}

