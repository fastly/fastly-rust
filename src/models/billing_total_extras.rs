/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BillingTotalExtras {
    /// The name of this extra cost.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Recurring monthly cost in USD. Not present if $0.0.
    #[serde(rename = "recurring", skip_serializing_if = "Option::is_none")]
    pub recurring: Option<f32>,
    /// Initial set up cost in USD. Not present if $0.0 or this is not the month the extra was added.
    #[serde(rename = "setup", skip_serializing_if = "Option::is_none")]
    pub setup: Option<f32>,
}

impl BillingTotalExtras {
    pub fn new() -> BillingTotalExtras {
        BillingTotalExtras {
            name: None,
            recurring: None,
            setup: None,
        }
    }
}


