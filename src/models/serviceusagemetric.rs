/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Serviceusagemetric {
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Box<String>>,
    /// Service ID associated with the usage.
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// Name of the service associated with the usage.
    #[serde(rename = "service_name", skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// The quantity of the usage for the billing period. Amount will be in the units provided in the parent object (e.g., a quantity of `1.3` with a unit of `gb` would have a usage amount of 1.3 gigabytes).
    #[serde(rename = "usage_units", skip_serializing_if = "Option::is_none")]
    pub usage_units: Option<f32>,
}

impl Serviceusagemetric {
    pub fn new() -> Serviceusagemetric {
        Serviceusagemetric {
            customer_id: None,
            service_id: None,
            service_name: None,
            usage_units: None,
        }
    }
}


