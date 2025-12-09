/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SuggestionAllOf {
    /// The suggested domain, consisting of a subdomain and zone.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The subdomain of the suggested domain.
    #[serde(rename = "subdomain", skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
    /// The zone of the suggested domain.
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    /// If present, the path is to be appended to the domain to complete the suggestion.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl SuggestionAllOf {
    pub fn new() -> SuggestionAllOf {
        SuggestionAllOf {
            domain: None,
            subdomain: None,
            zone: None,
            path: None,
        }
    }
}


