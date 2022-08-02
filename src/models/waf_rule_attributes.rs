/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafRuleAttributes {
    /// Corresponding ModSecurity rule ID.
    #[serde(rename = "modsec_rule_id", skip_serializing_if = "Option::is_none")]
    pub modsec_rule_id: Option<i32>,
    /// Rule publisher.
    #[serde(rename = "publisher", skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,
    /// The rule's [type](https://docs.fastly.com/en/guides/managing-rules-on-the-fastly-waf#understanding-the-types-of-rules).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
}

impl WafRuleAttributes {
    pub fn new() -> WafRuleAttributes {
        WafRuleAttributes {
            modsec_rule_id: None,
            publisher: None,
            _type: None,
        }
    }
}

/// Rule publisher.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Publisher {
    #[serde(rename = "fastly")]
    Fastly,
    #[serde(rename = "trustwave")]
    Trustwave,
    #[serde(rename = "owasp")]
    Owasp,
}

impl Default for Publisher {
    fn default() -> Publisher {
        Self::Fastly
    }
}
/// The rule's [type](https://docs.fastly.com/en/guides/managing-rules-on-the-fastly-waf#understanding-the-types-of-rules).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "score")]
    Score,
    #[serde(rename = "threshold")]
    Threshold,
}

impl Default for Type {
    fn default() -> Type {
        Self::Strict
    }
}

