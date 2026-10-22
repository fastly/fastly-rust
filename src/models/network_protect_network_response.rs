/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectNetworkResponse : A Network Protect network response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectNetworkResponse {
    /// Optional descriptive text for the network. This metadata does not change traffic handling. Null clears it on update.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A human-readable name, unique within the customer.
    #[serde(rename = "name")]
    pub name: String,
    /// The CIDR block to protect, masked to its network address. It cannot be changed after creation.
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// The backend-assigned rule set applied to this network.
    #[serde(rename = "rule_set_id", skip_serializing_if = "Option::is_none")]
    pub rule_set_id: Option<String>,
    /// Complete endpoint policy configuration. A network has at most one policy per region and may have no policies.
    #[serde(rename = "endpoint_policies")]
    pub endpoint_policies: Vec<crate::models::NetworkProtectEndpointPolicy>,
    /// Desired advertisement state, which is not the same as observed route state. Enabling it requires approved LOA, valid ROA, and a ready default-region delivery policy. A later ROA failure does not turn an existing advertisement off; losing LOA approval or the last ready default endpoint does.
    #[serde(rename = "advertise")]
    pub advertise: bool,
    /// A read-only unique identifier for a Network Protect resource.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "roa_status", skip_serializing_if = "Option::is_none")]
    pub roa_status: Option<crate::models::NetworkProtectRoaStatus>,
    /// When Fastly last completed a ROA check. Null while the status is pending. A failed check leaves this unchanged; an old value can indicate delayed or failing checks. Null also means the controller did not report a check time.
    #[serde(rename = "roa_checked_at", skip_serializing_if = "Option::is_none")]
    pub roa_checked_at: Option<String>,
    /// When the RPKI dataset used by the last check was built. This is not the same as `roa_checked_at`; a recent check can use an older dataset. Null while the status is pending or when no dataset build time was reported.
    #[serde(rename = "roa_data_as_of", skip_serializing_if = "Option::is_none")]
    pub roa_data_as_of: Option<String>,
    #[serde(rename = "loa_status", skip_serializing_if = "Option::is_none")]
    pub loa_status: Option<crate::models::NetworkProtectLoaStatus>,
    /// An ISO 8601 timestamp in UTC.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// An ISO 8601 timestamp in UTC.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl NetworkProtectNetworkResponse {
    /// A Network Protect network response.
    pub fn new(name: String, prefix: String, endpoint_policies: Vec<crate::models::NetworkProtectEndpointPolicy>, advertise: bool, id: String, created_at: String, updated_at: String) -> NetworkProtectNetworkResponse {
        NetworkProtectNetworkResponse {
            description: None,
            name,
            prefix,
            rule_set_id: None,
            endpoint_policies,
            advertise,
            id,
            roa_status: None,
            roa_checked_at: None,
            roa_data_as_of: None,
            loa_status: None,
            created_at,
            updated_at,
        }
    }
}


