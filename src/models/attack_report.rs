/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttackReport {
    /// ID of the workspace.
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the workspace.
    #[serde(rename = "name")]
    pub name: String,
    /// Total request count
    #[serde(rename = "total_count")]
    pub total_count: i32,
    /// Blocked request count
    #[serde(rename = "blocked_count")]
    pub blocked_count: i32,
    /// Flagged request count
    #[serde(rename = "flagged_count")]
    pub flagged_count: i32,
    /// Attack request count
    #[serde(rename = "attack_count")]
    pub attack_count: i32,
    /// Count of IPs that have been flagged
    #[serde(rename = "all_flagged_ip_count")]
    pub all_flagged_ip_count: i32,
    /// Count of currently flagged IPs
    #[serde(rename = "flagged_ip_count")]
    pub flagged_ip_count: i32,
    #[serde(rename = "top_attack_signals")]
    pub top_attack_signals: Vec<crate::models::AttackSignal>,
    #[serde(rename = "top_attack_sources")]
    pub top_attack_sources: Vec<crate::models::AttackSource>,
}

impl AttackReport {
    pub fn new(id: String, name: String, total_count: i32, blocked_count: i32, flagged_count: i32, attack_count: i32, all_flagged_ip_count: i32, flagged_ip_count: i32, top_attack_signals: Vec<crate::models::AttackSignal>, top_attack_sources: Vec<crate::models::AttackSource>) -> AttackReport {
        AttackReport {
            id,
            name,
            total_count,
            blocked_count,
            flagged_count,
            attack_count,
            all_flagged_ip_count,
            flagged_ip_count,
            top_attack_signals,
            top_attack_sources,
        }
    }
}


