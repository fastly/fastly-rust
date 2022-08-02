/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BillingResponseLineItem {
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(rename = "aria_invoice_id", skip_serializing_if = "Option::is_none")]
    pub aria_invoice_id: Option<Box<String>>,
    #[serde(rename = "client_service_id", skip_serializing_if = "Option::is_none")]
    pub client_service_id: Option<String>,
    #[serde(rename = "credit_coupon_code", skip_serializing_if = "Option::is_none")]
    pub credit_coupon_code: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "line_number", skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i32>,
    #[serde(rename = "plan_name", skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "plan_no", skip_serializing_if = "Option::is_none")]
    pub plan_no: Option<f32>,
    #[serde(rename = "rate_per_unit", skip_serializing_if = "Option::is_none")]
    pub rate_per_unit: Option<f32>,
    #[serde(rename = "rate_schedule_no", skip_serializing_if = "Option::is_none")]
    pub rate_schedule_no: Option<f32>,
    #[serde(rename = "rate_schedule_tier_no", skip_serializing_if = "Option::is_none")]
    pub rate_schedule_tier_no: Option<f32>,
    #[serde(rename = "service_name", skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "service_no", skip_serializing_if = "Option::is_none")]
    pub service_no: Option<f32>,
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<f32>,
    #[serde(rename = "usage_type_cd", skip_serializing_if = "Option::is_none")]
    pub usage_type_cd: Option<String>,
    #[serde(rename = "usage_type_no", skip_serializing_if = "Option::is_none")]
    pub usage_type_no: Option<f32>,
}

impl BillingResponseLineItem {
    pub fn new() -> BillingResponseLineItem {
        BillingResponseLineItem {
            created_at: None,
            deleted_at: None,
            updated_at: None,
            amount: None,
            aria_invoice_id: None,
            client_service_id: None,
            credit_coupon_code: None,
            description: None,
            id: None,
            line_number: None,
            plan_name: None,
            plan_no: None,
            rate_per_unit: None,
            rate_schedule_no: None,
            rate_schedule_tier_no: None,
            service_name: None,
            service_no: None,
            units: None,
            usage_type_cd: None,
            usage_type_no: None,
        }
    }
}


