/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BillingEstimateResponseAllOfLine {
    #[serde(rename = "plan_no", skip_serializing_if = "Option::is_none")]
    pub plan_no: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<f32>,
    #[serde(rename = "per_unit_cost", skip_serializing_if = "Option::is_none")]
    pub per_unit_cost: Option<f32>,
    #[serde(rename = "service_no", skip_serializing_if = "Option::is_none")]
    pub service_no: Option<f32>,
    #[serde(rename = "service_type", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(rename = "client_service_id", skip_serializing_if = "Option::is_none")]
    pub client_service_id: Option<String>,
    #[serde(rename = "client_plan_id", skip_serializing_if = "Option::is_none")]
    pub client_plan_id: Option<String>,
}

impl BillingEstimateResponseAllOfLine {
    pub fn new() -> BillingEstimateResponseAllOfLine {
        BillingEstimateResponseAllOfLine {
            plan_no: None,
            description: None,
            units: None,
            per_unit_cost: None,
            service_no: None,
            service_type: None,
            amount: None,
            client_service_id: None,
            client_plan_id: None,
        }
    }
}


