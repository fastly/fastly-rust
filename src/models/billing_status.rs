/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BillingStatus {
    /// What the current status of this invoice can be.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "sent_at", skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<Box<String>>,
}

impl BillingStatus {
    pub fn new() -> BillingStatus {
        BillingStatus {
            status: None,
            sent_at: None,
        }
    }
}

/// What the current status of this invoice can be.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Outstanding")]
    Outstanding,
    #[serde(rename = "Paid")]
    Paid,
    #[serde(rename = "MTD")]
    MTD,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

