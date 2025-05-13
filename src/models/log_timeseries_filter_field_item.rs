/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// LogTimeseriesFilterFieldItem : A filtering parameter.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogTimeseriesFilterFieldItem {
    /// The log field to which this filter should be applied.
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The comparison operator used for this filter.
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<crate::models::LogTimeseriesValueField>>,
}

impl LogTimeseriesFilterFieldItem {
    /// A filtering parameter.
    pub fn new() -> LogTimeseriesFilterFieldItem {
        LogTimeseriesFilterFieldItem {
            field: None,
            operator: None,
            value: None,
        }
    }
}

/// The comparison operator used for this filter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "ends-with")]
    EndsWith,
    #[serde(rename = "in")]
    _in,
    #[serde(rename = "not_in")]
    NotIn,
    #[serde(rename = "gt")]
    Gt,
    #[serde(rename = "gte")]
    Gte,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lte")]
    Lte,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Eq
    }
}

