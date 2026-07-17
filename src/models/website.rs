/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Website {
    /// Unique website identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Website domain
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// IDs of pages associated with this website
    #[serde(rename = "page_ids", skip_serializing_if = "Option::is_none")]
    pub page_ids: Option<Vec<String>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl Website {
    pub fn new() -> Website {
        Website {
            id: None,
            domain: None,
            page_ids: None,
            created_at: None,
            updated_at: None,
        }
    }
}


