/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttackSignal {
    /// Name of the attack signal tag
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    /// Count of requests with this attack signal
    #[serde(rename = "tag_count")]
    pub tag_count: i32,
    /// Total number of attacks considered
    #[serde(rename = "total_count")]
    pub total_count: i32,
}

impl AttackSignal {
    pub fn new(tag_name: String, tag_count: i32, total_count: i32) -> AttackSignal {
        AttackSignal {
            tag_name,
            tag_count,
            total_count,
        }
    }
}


