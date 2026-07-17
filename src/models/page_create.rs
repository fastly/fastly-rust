/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PageCreate {
    #[serde(rename = "website_id")]
    pub website_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "notifications", skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<crate::models::Notification>>,
    #[serde(rename = "paths", skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
}

impl PageCreate {
    pub fn new(website_id: String, name: String) -> PageCreate {
        PageCreate {
            website_id,
            name,
            description: None,
            notifications: None,
            paths: None,
        }
    }
}


