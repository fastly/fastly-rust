/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContactResponse {
    /// The alphanumeric string representing the user for this customer contact.
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The type of contact.
    #[serde(rename = "contact_type", skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<ContactType>,
    /// The name of this contact, when user_id is not provided.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The email of this contact, when a user_id is not provided.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The phone number for this contact. Required for primary, technical, and security contact types.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The alphanumeric string representing the customer for this customer contact.
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<String>>,
}

impl ContactResponse {
    pub fn new() -> ContactResponse {
        ContactResponse {
            user_id: None,
            contact_type: None,
            name: None,
            email: None,
            phone: None,
            customer_id: None,
            created_at: None,
            deleted_at: None,
            updated_at: None,
            id: None,
        }
    }
}

/// The type of contact.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContactType {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "billing")]
    Billing,
    #[serde(rename = "technical")]
    Technical,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "emergency")]
    Emergency,
    #[serde(rename = "general compliance")]
    GeneralCompliance,
}

impl Default for ContactType {
    fn default() -> ContactType {
        Self::Primary
    }
}

