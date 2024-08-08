/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BillingBandwidthTiers {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<f32>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    #[serde(rename = "discounted_price", skip_serializing_if = "Option::is_none")]
    pub discounted_price: Option<f32>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f32>,
}

impl BillingBandwidthTiers {
    pub fn new() -> BillingBandwidthTiers {
        BillingBandwidthTiers {
            name: None,
            units: None,
            price: None,
            discounted_price: None,
            total: None,
        }
    }
}


