/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// SuccessfulResponseAsObject : All attributes for a domain response



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SuccessfulResponseAsObject {
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Domain Identifier (UUID).
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The fully-qualified domain name for your domain. Can be created, but not updated.
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    /// The `service_id` associated with your domain or `null` if there is no association.
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// A freeform descriptive note.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Denotes if the domain has at least one TLS activation or not.
    #[serde(rename = "activated", skip_serializing_if = "Option::is_none")]
    pub activated: Option<bool>,
    /// Denotes that the customer has proven ownership over the domain by obtaining a Fastly-managed TLS certificate for it or by providing a their own TLS certificate from a publicly-trusted CA that includes the domain or matching wildcard.     
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl SuccessfulResponseAsObject {
    /// All attributes for a domain response
    pub fn new() -> SuccessfulResponseAsObject {
        SuccessfulResponseAsObject {
            created_at: None,
            updated_at: None,
            id: None,
            fqdn: None,
            service_id: None,
            description: None,
            activated: None,
            verified: None,
        }
    }
}


