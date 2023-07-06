/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoricalDomains {
    /// Whether or not we were able to successfully execute the query.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::HistoricalDomainsMeta>>,
    /// If the query was not successful, this will provide a string that explains why.
    #[serde(rename = "msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    /// A list of [entries](#entry-data-model), each representing one unique combination of dimensions, such as domain, region, or POP.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::HistoricalDomainsData>>,
}

impl HistoricalDomains {
    pub fn new() -> HistoricalDomains {
        HistoricalDomains {
            status: None,
            meta: None,
            msg: None,
            data: None,
        }
    }
}


