/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Serviceusagetype {
    /// The product identifier associated with the usage type. This corresponds to a Fastly product offering.
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Full name of the product usage type as it might appear on a customer's invoice.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Serviceusagetype {
    pub fn new() -> Serviceusagetype {
        Serviceusagetype {
            product_id: None,
            name: None,
        }
    }
}


