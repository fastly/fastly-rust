/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApexRedirectAllOf {
    /// HTTP status code used to redirect the client.
    #[serde(rename = "status_code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<StatusCode>,
    /// Array of apex domains that should redirect to their WWW subdomain.
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// Revision number of the apex redirect feature implementation. Defaults to the most recent revision.
    #[serde(rename = "feature_revision", skip_serializing_if = "Option::is_none")]
    pub feature_revision: Option<i32>,
}

impl ApexRedirectAllOf {
    pub fn new() -> ApexRedirectAllOf {
        ApexRedirectAllOf {
            status_code: None,
            domains: None,
            feature_revision: None,
        }
    }
}

/// HTTP status code used to redirect the client.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCode {
    #[serde(rename = "301")]
    StatusCode301,
    #[serde(rename = "302")]
    StatusCode302,
    #[serde(rename = "307")]
    StatusCode307,
    #[serde(rename = "308")]
    StatusCode308,
}

impl Default for StatusCode {
    fn default() -> StatusCode {
        Self::StatusCode301
    }
}

