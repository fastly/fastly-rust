/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogInsightsDimensionAttributes {
    /// The rate at which the value in the current dimension occurs.
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
    /// The cache hit ratio for the country.
    #[serde(rename = "country_chr", skip_serializing_if = "Option::is_none")]
    pub country_chr: Option<f32>,
    /// The error rate for the country.
    #[serde(rename = "country_error_rate", skip_serializing_if = "Option::is_none")]
    pub country_error_rate: Option<f32>,
    /// This country's percentage of the total requests.
    #[serde(rename = "country_request_rate", skip_serializing_if = "Option::is_none")]
    pub country_request_rate: Option<f32>,
}

impl LogInsightsDimensionAttributes {
    pub fn new() -> LogInsightsDimensionAttributes {
        LogInsightsDimensionAttributes {
            rate: None,
            country_chr: None,
            country_error_rate: None,
            country_request_rate: None,
        }
    }
}


