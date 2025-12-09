/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// Status : All attributes for a domain status.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Status {
    /// The scope provided in the status request.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// The domain provided in the status request.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The zone of the domain provided of the status request.
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    /// A space-delimited string of the varying statuses associated with the domain provided.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// A space-delimited string of the varying tags associated with the domain provided.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(rename = "offers", skip_serializing_if = "Option::is_none")]
    pub offers: Option<Vec<crate::models::Offer>>,
}

impl Status {
    /// All attributes for a domain status.
    pub fn new() -> Status {
        Status {
            scope: None,
            domain: None,
            zone: None,
            status: None,
            tags: None,
            offers: None,
        }
    }
}


