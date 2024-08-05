/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceusagemetricsData {
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Box<String>>,
    /// Date and time (in ISO 8601 format) for initiation point of a billing cycle, signifying the start of charges for a service or subscription.
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Date and time (in ISO 8601 format) for termination point of a billing cycle, signifying the end of charges for a service or subscription.
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The usage type identifier for the usage. This is a single, billable metric for the product.
    #[serde(rename = "usage_type", skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
    /// The unit for the usage as shown on an invoice. If there is no explicit unit, this field will be \"unit\" (e.g., a request with `product_id` of 'cdn_usage' and `usage_type` of 'North America Requests' has no unit, and will return \"unit\").
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::models::Serviceusagemetric>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Metadata>>,
}

impl ServiceusagemetricsData {
    pub fn new() -> ServiceusagemetricsData {
        ServiceusagemetricsData {
            customer_id: None,
            start_time: None,
            end_time: None,
            usage_type: None,
            unit: None,
            details: None,
            meta: None,
        }
    }
}


