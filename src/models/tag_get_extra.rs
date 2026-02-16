/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TagGetExtra {
    /// The unique identifier of the operation tag.
    #[serde(rename = "id")]
    pub id: String,
    /// The number of operations associated with this operation tag.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The date and time the operation tag was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The date and time the operation tag was last updated.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl TagGetExtra {
    pub fn new(id: String) -> TagGetExtra {
        TagGetExtra {
            id,
            count: None,
            created_at: None,
            updated_at: None,
        }
    }
}


