/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BillingEstimateResponse {
    /// Date and time in ISO 8601 format.
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "invoice_id", skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<Box<String>>,
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Box<String>>,
    /// The current state of our third-party billing vendor. One of `up` or `down`.
    #[serde(rename = "vendor_state", skip_serializing_if = "Option::is_none")]
    pub vendor_state: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::BillingStatus>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<Box<crate::models::BillingTotal>>,
    /// Breakdown of regional data for products that are region based.
    #[serde(rename = "regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "lines", skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<crate::models::BillingEstimateResponseAllOfLines>>,
}

impl BillingEstimateResponse {
    pub fn new() -> BillingEstimateResponse {
        BillingEstimateResponse {
            end_time: None,
            start_time: None,
            invoice_id: None,
            customer_id: None,
            vendor_state: None,
            status: None,
            total: None,
            regions: None,
            lines: None,
        }
    }
}


