/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// RelationshipMutualAuthentication : The [Mutual Authentication](/reference/api/tls/mutual-tls/authentication/) for client-to-server authentication. Optional.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipMutualAuthentication {
    #[serde(rename = "mutual_authentication", skip_serializing_if = "Option::is_none")]
    pub mutual_authentication: Option<Box<crate::models::RelationshipMutualAuthenticationMutualAuthentication>>,
}

impl RelationshipMutualAuthentication {
    /// The [Mutual Authentication](/reference/api/tls/mutual-tls/authentication/) for client-to-server authentication. Optional.
    pub fn new() -> RelationshipMutualAuthentication {
        RelationshipMutualAuthentication {
            mutual_authentication: None,
        }
    }
}


