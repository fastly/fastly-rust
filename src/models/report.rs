/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Report {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "blocked_uri", skip_serializing_if = "Option::is_none")]
    pub blocked_uri: Option<String>,
    #[serde(rename = "document_uri", skip_serializing_if = "Option::is_none")]
    pub document_uri: Option<String>,
    #[serde(rename = "violated_directive", skip_serializing_if = "Option::is_none")]
    pub violated_directive: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl Report {
    pub fn new() -> Report {
        Report {
            id: None,
            policy_id: None,
            blocked_uri: None,
            document_uri: None,
            violated_directive: None,
            created_at: None,
        }
    }
}


