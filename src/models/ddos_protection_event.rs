/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DdosProtectionEvent {
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Unique ID of the event.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A human-readable name for the event.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Alphanumeric string identifying the customer.
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Alphanumeric string identifying the service.
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "started_at", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "ended_at", skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
}

impl DdosProtectionEvent {
    pub fn new() -> DdosProtectionEvent {
        DdosProtectionEvent {
            created_at: None,
            updated_at: None,
            id: None,
            name: None,
            customer_id: None,
            service_id: None,
            started_at: None,
            ended_at: None,
        }
    }
}


