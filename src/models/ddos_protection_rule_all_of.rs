/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DdosProtectionRuleAllOf {
    /// Unique ID of the rule.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A human-readable name for the rule.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Action types for a rule. Supported action values are default, block, log, off. The default action value follows the current protection mode of the associated service.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Alphanumeric string identifying the customer.
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Alphanumeric string identifying the service.
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// Source IP address attribute.
    #[serde(rename = "source_ip", skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    /// Country code attribute.
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Host attribute.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// ASN attribute.
    #[serde(rename = "asn", skip_serializing_if = "Option::is_none")]
    pub asn: Option<String>,
    /// Source IP prefix attribute.
    #[serde(rename = "source_ip_prefix", skip_serializing_if = "Option::is_none")]
    pub source_ip_prefix: Option<String>,
    /// Attribute category for additional, unlisted attributes used in a rule.
    #[serde(rename = "additional_attributes", skip_serializing_if = "Option::is_none")]
    pub additional_attributes: Option<Vec<String>>,
}

impl DdosProtectionRuleAllOf {
    pub fn new() -> DdosProtectionRuleAllOf {
        DdosProtectionRuleAllOf {
            id: None,
            name: None,
            action: None,
            customer_id: None,
            service_id: None,
            source_ip: None,
            country_code: None,
            host: None,
            asn: None,
            source_ip_prefix: None,
            additional_attributes: None,
        }
    }
}


