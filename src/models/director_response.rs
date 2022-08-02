/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DirectorResponse {
    /// List of backends associated to a director.
    #[serde(rename = "backends", skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<crate::models::Backend>>,
    /// Unused.
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    /// A freeform descriptive note.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Name for the Director.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The percentage of capacity that needs to be up for a director to be considered up. `0` to `100`.
    #[serde(rename = "quorum", skip_serializing_if = "Option::is_none")]
    pub quorum: Option<i32>,
    /// Selected POP to serve as a shield for the backends. Defaults to `null` meaning no origin shielding if not set. Refer to the [POPs API endpoint](/reference/api/utils/pops/) to get a list of available POPs used for shielding.
    #[serde(rename = "shield", skip_serializing_if = "Option::is_none")]
    pub shield: Option<String>,
    /// What type of load balance group to use.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// How many backends to search if it fails.
    #[serde(rename = "retries", skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<String>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<i32>>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl DirectorResponse {
    pub fn new() -> DirectorResponse {
        DirectorResponse {
            backends: None,
            capacity: None,
            comment: None,
            name: None,
            quorum: None,
            shield: None,
            _type: None,
            retries: None,
            service_id: None,
            version: None,
            created_at: None,
            deleted_at: None,
            updated_at: None,
        }
    }
}

/// What type of load balance group to use.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "1")]
    TypeRandom,
    #[serde(rename = "3")]
    TypeHash,
    #[serde(rename = "4")]
    TypeClient,
}

impl Default for Type {
    fn default() -> Type {
        Self::TypeRandom
    }
}

