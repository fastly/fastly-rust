/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectPrefixListResponse : A Network Protect prefix-list response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectPrefixListResponse {
    /// A human-readable name, unique within the customer.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "family")]
    pub family: crate::models::NetworkProtectPrefixListFamily,
    /// Complete canonical list contents. Bare addresses are canonicalized, CIDRs are masked, equivalent entries are deduplicated, and overlaps are retained.
    #[serde(rename = "addresses")]
    pub addresses: Vec<String>,
    /// A read-only unique identifier for a Network Protect resource.
    #[serde(rename = "id")]
    pub id: String,
    /// An ISO 8601 timestamp in UTC.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// An ISO 8601 timestamp in UTC.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl NetworkProtectPrefixListResponse {
    /// A Network Protect prefix-list response.
    pub fn new(name: String, family: crate::models::NetworkProtectPrefixListFamily, addresses: Vec<String>, id: String, created_at: String, updated_at: String) -> NetworkProtectPrefixListResponse {
        NetworkProtectPrefixListResponse {
            name,
            family,
            addresses,
            id,
            created_at,
            updated_at,
        }
    }
}


