/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcProviderConnectionListItemResponseExtra {
    /// The date and time the connection was deleted, in RFC 3339 format. Present only for deleted connections, which are returned only when `include_deleted` is `true`.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl ArcProviderConnectionListItemResponseExtra {
    pub fn new() -> ArcProviderConnectionListItemResponseExtra {
        ArcProviderConnectionListItemResponseExtra {
            deleted_at: None,
        }
    }
}


