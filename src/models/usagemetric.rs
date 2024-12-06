/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Usagemetric {
    /// The year and month of the usage element.
    #[serde(rename = "month", skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
    /// The usage type identifier for the usage. This is a single, billable metric for the product.
    #[serde(rename = "usage_type", skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
    /// Full name of the product usage type as it might appear on a customer's invoice.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The geographical area applicable for regionally based products.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The unit for the usage as shown on an invoice. If there is no explicit unit, this field will be \"unit\" (e.g., a request with `product_id` of 'cdn_usage' and `usage_type` of 'North America Requests' has no unit, and will return \"unit\").
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// The quantity of the usage for the product.
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f32>,
    /// The raw units measured for the product.
    #[serde(rename = "raw_quantity", skip_serializing_if = "Option::is_none")]
    pub raw_quantity: Option<f32>,
    /// The product identifier associated with the usage type. This corresponds to a Fastly product offering.
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The date when the usage metric was last updated.
    #[serde(rename = "last_updated_at", skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
}

impl Usagemetric {
    pub fn new() -> Usagemetric {
        Usagemetric {
            month: None,
            usage_type: None,
            name: None,
            region: None,
            unit: None,
            quantity: None,
            raw_quantity: None,
            product_id: None,
            last_updated_at: None,
        }
    }
}


