/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// RequestBodyForCreate : All attributes for creating a domain



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestBodyForCreate {
    /// The fully-qualified domain name for your domain. Can be created, but not updated.
    #[serde(rename = "fqdn")]
    pub fqdn: String,
    /// The `service_id` associated with your domain or `null` if there is no association.
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// A freeform descriptive note.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl RequestBodyForCreate {
    /// All attributes for creating a domain
    pub fn new(fqdn: String) -> RequestBodyForCreate {
        RequestBodyForCreate {
            fqdn,
            service_id: None,
            description: None,
        }
    }
}


