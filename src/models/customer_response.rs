/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomerResponse {
    /// The alphanumeric string representing the primary billing contact.
    #[serde(rename = "billing_contact_id", skip_serializing_if = "Option::is_none")]
    pub billing_contact_id: Option<String>,
    /// Customer's current network revenue type.
    #[serde(rename = "billing_network_type", skip_serializing_if = "Option::is_none")]
    pub billing_network_type: Option<BillingNetworkType>,
    /// Used for adding purchased orders to customer's account.
    #[serde(rename = "billing_ref", skip_serializing_if = "Option::is_none")]
    pub billing_ref: Option<String>,
    /// Whether this customer can view or edit wordpress.
    #[serde(rename = "can_configure_wordpress", skip_serializing_if = "Option::is_none")]
    pub can_configure_wordpress: Option<bool>,
    /// Whether this customer can reset passwords.
    #[serde(rename = "can_reset_passwords", skip_serializing_if = "Option::is_none")]
    pub can_reset_passwords: Option<bool>,
    /// Whether this customer can upload VCL.
    #[serde(rename = "can_upload_vcl", skip_serializing_if = "Option::is_none")]
    pub can_upload_vcl: Option<bool>,
    /// Specifies whether 2FA is forced or not forced on the customer account. Logs out non-2FA users once 2FA is force enabled.
    #[serde(rename = "force_2fa", skip_serializing_if = "Option::is_none")]
    pub force_2fa: Option<bool>,
    /// Specifies whether SSO is forced or not forced on the customer account.
    #[serde(rename = "force_sso", skip_serializing_if = "Option::is_none")]
    pub force_sso: Option<bool>,
    /// Specifies whether the account has access or does not have access to the account panel.
    #[serde(rename = "has_account_panel", skip_serializing_if = "Option::is_none")]
    pub has_account_panel: Option<bool>,
    /// Specifies whether the account has access or does not have access to the improved events.
    #[serde(rename = "has_improved_events", skip_serializing_if = "Option::is_none")]
    pub has_improved_events: Option<bool>,
    /// Whether this customer can view or edit the SSL config.
    #[serde(rename = "has_improved_ssl_config", skip_serializing_if = "Option::is_none")]
    pub has_improved_ssl_config: Option<bool>,
    /// Specifies whether the account has enabled or not enabled openstack logging.
    #[serde(rename = "has_openstack_logging", skip_serializing_if = "Option::is_none")]
    pub has_openstack_logging: Option<bool>,
    /// Specifies whether the account can edit PCI for a service.
    #[serde(rename = "has_pci", skip_serializing_if = "Option::is_none")]
    pub has_pci: Option<bool>,
    /// Specifies whether PCI passwords are required for the account.
    #[serde(rename = "has_pci_passwords", skip_serializing_if = "Option::is_none")]
    pub has_pci_passwords: Option<bool>,
    /// The range of IP addresses authorized to access the customer account.
    #[serde(rename = "ip_whitelist", skip_serializing_if = "Option::is_none")]
    pub ip_whitelist: Option<String>,
    /// The alphanumeric string identifying the account's legal contact.
    #[serde(rename = "legal_contact_id", skip_serializing_if = "Option::is_none")]
    pub legal_contact_id: Option<String>,
    /// The name of the customer, generally the company name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The alphanumeric string identifying the account owner.
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// The phone number associated with the account.
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The postal address associated with the account.
    #[serde(rename = "postal_address", skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<String>,
    /// The pricing plan this customer is under.
    #[serde(rename = "pricing_plan", skip_serializing_if = "Option::is_none")]
    pub pricing_plan: Option<String>,
    /// The alphanumeric string identifying the pricing plan.
    #[serde(rename = "pricing_plan_id", skip_serializing_if = "Option::is_none")]
    pub pricing_plan_id: Option<String>,
    /// The alphanumeric string identifying the account's security contact.
    #[serde(rename = "security_contact_id", skip_serializing_if = "Option::is_none")]
    pub security_contact_id: Option<String>,
    /// The alphanumeric string identifying the account's technical contact.
    #[serde(rename = "technical_contact_id", skip_serializing_if = "Option::is_none")]
    pub technical_contact_id: Option<String>,
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

impl CustomerResponse {
    pub fn new() -> CustomerResponse {
        CustomerResponse {
            billing_contact_id: None,
            billing_network_type: None,
            billing_ref: None,
            can_configure_wordpress: None,
            can_reset_passwords: None,
            can_upload_vcl: None,
            force_2fa: None,
            force_sso: None,
            has_account_panel: None,
            has_improved_events: None,
            has_improved_ssl_config: None,
            has_openstack_logging: None,
            has_pci: None,
            has_pci_passwords: None,
            ip_whitelist: None,
            legal_contact_id: None,
            name: None,
            owner_id: None,
            phone_number: None,
            postal_address: None,
            pricing_plan: None,
            pricing_plan_id: None,
            security_contact_id: None,
            technical_contact_id: None,
            created_at: None,
            deleted_at: None,
            updated_at: None,
            id: None,
        }
    }
}

/// Customer's current network revenue type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillingNetworkType {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl Default for BillingNetworkType {
    fn default() -> BillingNetworkType {
        Self::Public
    }
}

