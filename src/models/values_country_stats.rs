/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValuesCountryStats {
    /// The client's country subdivision code as defined by ISO 3166-2.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The cache hit ratio for the region.
    #[serde(rename = "region_chr", skip_serializing_if = "Option::is_none")]
    pub region_chr: Option<f32>,
    /// The error rate for the region.
    #[serde(rename = "region_error_rate", skip_serializing_if = "Option::is_none")]
    pub region_error_rate: Option<f32>,
}

impl ValuesCountryStats {
    pub fn new() -> ValuesCountryStats {
        ValuesCountryStats {
            region: None,
            region_chr: None,
            region_error_rate: None,
        }
    }
}


