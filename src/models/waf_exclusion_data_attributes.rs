/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafExclusionDataAttributes {
    /// A conditional expression in VCL used to determine if the condition is met.
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// The type of exclusion.
    #[serde(rename = "exclusion_type", skip_serializing_if = "Option::is_none")]
    pub exclusion_type: Option<ExclusionType>,
    /// Whether to generate a log upon matching.
    #[serde(rename = "logging", skip_serializing_if = "Option::is_none")]
    pub logging: Option<bool>,
    /// Name of the exclusion.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A numeric ID identifying a WAF exclusion.
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    /// The variable to exclude. An optional selector can be specified after the variable separated by a colon (`:`) to restrict the variable to a particular parameter. Required for `exclusion_type=variable`.
    #[serde(rename = "variable", skip_serializing_if = "Option::is_none")]
    pub variable: Option<Variable>,
}

impl WafExclusionDataAttributes {
    pub fn new() -> WafExclusionDataAttributes {
        WafExclusionDataAttributes {
            condition: None,
            exclusion_type: None,
            logging: None,
            name: None,
            number: None,
            variable: None,
        }
    }
}

/// The type of exclusion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExclusionType {
    #[serde(rename = "rule")]
    Rule,
    #[serde(rename = "variable")]
    Variable,
    #[serde(rename = "waf")]
    Waf,
}

impl Default for ExclusionType {
    fn default() -> ExclusionType {
        Self::Rule
    }
}
/// The variable to exclude. An optional selector can be specified after the variable separated by a colon (`:`) to restrict the variable to a particular parameter. Required for `exclusion_type=variable`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Variable {
    #[serde(rename = "req.cookies")]
    ReqCookies,
    #[serde(rename = "req.headers")]
    ReqHeaders,
    #[serde(rename = "req.post")]
    ReqPost,
    #[serde(rename = "req.post_filename")]
    ReqPostFilename,
    #[serde(rename = "req.qs")]
    ReqQs,
    #[serde(rename = "null")]
    Null,
}

impl Default for Variable {
    fn default() -> Variable {
        Self::ReqCookies
    }
}

