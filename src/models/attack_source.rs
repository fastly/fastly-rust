/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttackSource {
    /// Country code of the attack source
    #[serde(rename = "country_code")]
    pub country_code: String,
    /// Name of the country
    #[serde(rename = "country_name")]
    pub country_name: String,
    /// Number of requests from this country
    #[serde(rename = "request_count")]
    pub request_count: i32,
    /// Total number of attacks considered
    #[serde(rename = "total_count")]
    pub total_count: i32,
}

impl AttackSource {
    pub fn new(country_code: String, country_name: String, request_count: i32, total_count: i32) -> AttackSource {
        AttackSource {
            country_code,
            country_name,
            request_count,
            total_count,
        }
    }
}


