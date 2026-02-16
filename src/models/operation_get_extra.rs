/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OperationGetExtra {
    /// The unique identifier of the operation.
    #[serde(rename = "id")]
    pub id: String,
    /// The timestamp when the operation was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The timestamp when the operation was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The timestamp when the operation was last seen in traffic.
    #[serde(rename = "last_seen_at", skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
}

impl OperationGetExtra {
    pub fn new(id: String, updated_at: String) -> OperationGetExtra {
        OperationGetExtra {
            id,
            created_at: None,
            updated_at,
            last_seen_at: None,
        }
    }
}


