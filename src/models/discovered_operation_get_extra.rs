/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DiscoveredOperationGetExtra {
    /// The current status of the operation.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The timestamp when the operation was last updated.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The timestamp when the operation was last seen in traffic.
    #[serde(rename = "last_seen_at", skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
}

impl DiscoveredOperationGetExtra {
    pub fn new() -> DiscoveredOperationGetExtra {
        DiscoveredOperationGetExtra {
            status: None,
            updated_at: None,
            last_seen_at: None,
        }
    }
}

/// The current status of the operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "DISCOVERED")]
    DISCOVERED,
    #[serde(rename = "SAVED")]
    SAVED,
    #[serde(rename = "IGNORED")]
    IGNORED,
}

impl Default for Status {
    fn default() -> Status {
        Self::DISCOVERED
    }
}

