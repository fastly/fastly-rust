/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OperationCreateExtra {
    /// The status to assign to the operation. Defaults to SAVED if omitted.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl OperationCreateExtra {
    pub fn new() -> OperationCreateExtra {
        OperationCreateExtra {
            status: None,
        }
    }
}

/// The status to assign to the operation. Defaults to SAVED if omitted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "SAVED")]
    SAVED,
    #[serde(rename = "IGNORED")]
    IGNORED,
}

impl Default for Status {
    fn default() -> Status {
        Self::SAVED
    }
}

