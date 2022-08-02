/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// BillingTotal : Complete summary of the billing information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BillingTotal {
    /// The total amount of bandwidth used this month (See bandwidth_units for measurement).
    #[serde(rename = "bandwidth", skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<f32>,
    /// The cost of the bandwidth used this month in USD.
    #[serde(rename = "bandwidth_cost", skip_serializing_if = "Option::is_none")]
    pub bandwidth_cost: Option<f32>,
    /// Bandwidth measurement units based on billing plan.
    #[serde(rename = "bandwidth_units", skip_serializing_if = "Option::is_none")]
    pub bandwidth_units: Option<String>,
    /// The final amount to be paid.
    #[serde(rename = "cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<f32>,
    /// Total incurred cost plus extras cost.
    #[serde(rename = "cost_before_discount", skip_serializing_if = "Option::is_none")]
    pub cost_before_discount: Option<f32>,
    /// Calculated discount rate.
    #[serde(rename = "discount", skip_serializing_if = "Option::is_none")]
    pub discount: Option<f32>,
    /// A list of any extras for this invoice.
    #[serde(rename = "extras", skip_serializing_if = "Option::is_none")]
    pub extras: Option<Vec<crate::models::BillingTotalExtras>>,
    /// Total cost of all extras.
    #[serde(rename = "extras_cost", skip_serializing_if = "Option::is_none")]
    pub extras_cost: Option<f32>,
    /// The total cost of bandwidth and requests used this month.
    #[serde(rename = "incurred_cost", skip_serializing_if = "Option::is_none")]
    pub incurred_cost: Option<f32>,
    /// How much over the plan minimum has been incurred.
    #[serde(rename = "overage", skip_serializing_if = "Option::is_none")]
    pub overage: Option<f32>,
    /// The short code the plan this invoice was generated under.
    #[serde(rename = "plan_code", skip_serializing_if = "Option::is_none")]
    pub plan_code: Option<String>,
    /// The minimum cost of this plan.
    #[serde(rename = "plan_minimum", skip_serializing_if = "Option::is_none")]
    pub plan_minimum: Option<f32>,
    /// The name of the plan this invoice was generated under.
    #[serde(rename = "plan_name", skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// The total number of requests used this month.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Option<f32>,
    /// The cost of the requests used this month.
    #[serde(rename = "requests_cost", skip_serializing_if = "Option::is_none")]
    pub requests_cost: Option<f32>,
    /// Payment terms. Almost always Net15.
    #[serde(rename = "terms", skip_serializing_if = "Option::is_none")]
    pub terms: Option<String>,
}

impl BillingTotal {
    /// Complete summary of the billing information.
    pub fn new() -> BillingTotal {
        BillingTotal {
            bandwidth: None,
            bandwidth_cost: None,
            bandwidth_units: None,
            cost: None,
            cost_before_discount: None,
            discount: None,
            extras: None,
            extras_cost: None,
            incurred_cost: None,
            overage: None,
            plan_code: None,
            plan_minimum: None,
            plan_name: None,
            requests: None,
            requests_cost: None,
            terms: None,
        }
    }
}


