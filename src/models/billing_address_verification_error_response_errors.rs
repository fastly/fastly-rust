/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BillingAddressVerificationErrorResponseErrors {
    /// The error type.
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(rename = "status")]
    pub status: f32,
    #[serde(rename = "candidates", skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<crate::models::BillingAddressAttributes>>,
}

impl BillingAddressVerificationErrorResponseErrors {
    pub fn new(_type: String, title: String, detail: String, status: f32) -> BillingAddressVerificationErrorResponseErrors {
        BillingAddressVerificationErrorResponseErrors {
            _type,
            title,
            detail,
            status,
            candidates: None,
        }
    }
}


