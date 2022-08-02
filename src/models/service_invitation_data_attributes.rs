/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceInvitationDataAttributes {
    /// The permission the accepting user will have in relation to the service.
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
}

impl ServiceInvitationDataAttributes {
    pub fn new() -> ServiceInvitationDataAttributes {
        ServiceInvitationDataAttributes {
            permission: None,
        }
    }
}

/// The permission the accepting user will have in relation to the service.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "read_only")]
    ReadOnly,
    #[serde(rename = "purge_select")]
    PurgeSelect,
    #[serde(rename = "purge_all")]
    PurgeAll,
}

impl Default for Permission {
    fn default() -> Permission {
        Self::Full
    }
}

