/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// HistoricalMeta : Meta information about the scope of the query in a human readable format.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoricalMeta {
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "by", skip_serializing_if = "Option::is_none")]
    pub by: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl HistoricalMeta {
    /// Meta information about the scope of the query in a human readable format.
    pub fn new() -> HistoricalMeta {
        HistoricalMeta {
            to: None,
            from: None,
            by: None,
            region: None,
        }
    }
}


