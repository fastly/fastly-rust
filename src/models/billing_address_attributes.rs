/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BillingAddressAttributes {
    /// The first address line.
    #[serde(rename = "address_1", skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    /// The second address line.
    #[serde(rename = "address_2", skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    /// The city name.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// ISO 3166-1 two-letter country code.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Other locality.
    #[serde(rename = "locality", skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// Postal code (ZIP code for US addresses).
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The state or province name.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Box<String>>,
}

impl BillingAddressAttributes {
    pub fn new() -> BillingAddressAttributes {
        BillingAddressAttributes {
            address_1: None,
            address_2: None,
            city: None,
            country: None,
            locality: None,
            postal_code: None,
            state: None,
            customer_id: None,
        }
    }
}


