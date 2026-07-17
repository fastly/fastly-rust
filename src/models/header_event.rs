/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HeaderEvent {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "header_name", skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
    #[serde(rename = "old_value", skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[serde(rename = "new_value", skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    #[serde(rename = "changed_at", skip_serializing_if = "Option::is_none")]
    pub changed_at: Option<String>,
}

impl HeaderEvent {
    pub fn new() -> HeaderEvent {
        HeaderEvent {
            id: None,
            header_name: None,
            old_value: None,
            new_value: None,
            changed_at: None,
        }
    }
}


