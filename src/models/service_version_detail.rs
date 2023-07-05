/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceVersionDetail {
    /// Whether this is the active version or not.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A freeform descriptive note.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Unused at this time.
    #[serde(rename = "deployed", skip_serializing_if = "Option::is_none")]
    pub deployed: Option<bool>,
    /// Whether this version is locked or not. Objects can not be added or edited on locked versions.
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// The number of this version.
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    /// Unused at this time.
    #[serde(rename = "staging", skip_serializing_if = "Option::is_none")]
    pub staging: Option<bool>,
    /// Unused at this time.
    #[serde(rename = "testing", skip_serializing_if = "Option::is_none")]
    pub testing: Option<bool>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<String>>,
    /// List of backends associated to this service.
    #[serde(rename = "backends", skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<crate::models::BackendResponse>>,
    /// List of cache settings associated to this service.
    #[serde(rename = "cache_settings", skip_serializing_if = "Option::is_none")]
    pub cache_settings: Option<Vec<crate::models::CacheSettingResponse>>,
    /// List of conditions associated to this service.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::ConditionResponse>>,
    /// List of directors associated to this service.
    #[serde(rename = "directors", skip_serializing_if = "Option::is_none")]
    pub directors: Option<Vec<crate::models::Director>>,
    /// List of domains associated to this service.
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<crate::models::DomainResponse>>,
    /// List of gzip rules associated to this service.
    #[serde(rename = "gzips", skip_serializing_if = "Option::is_none")]
    pub gzips: Option<Vec<crate::models::GzipResponse>>,
    /// List of headers associated to this service.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<crate::models::HeaderResponse>>,
    /// List of healthchecks associated to this service.
    #[serde(rename = "healthchecks", skip_serializing_if = "Option::is_none")]
    pub healthchecks: Option<Vec<crate::models::HealthcheckResponse>>,
    /// List of request settings for this service.
    #[serde(rename = "request_settings", skip_serializing_if = "Option::is_none")]
    pub request_settings: Option<Vec<crate::models::RequestSettingsResponse>>,
    /// List of response objects for this service.
    #[serde(rename = "response_objects", skip_serializing_if = "Option::is_none")]
    pub response_objects: Option<Vec<crate::models::ResponseObjectResponse>>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<crate::models::VersionDetailSettings>>,
    /// List of VCL snippets for this service.
    #[serde(rename = "snippets", skip_serializing_if = "Option::is_none")]
    pub snippets: Option<Vec<crate::models::SchemasSnippetResponse>>,
    /// List of VCL files for this service.
    #[serde(rename = "vcls", skip_serializing_if = "Option::is_none")]
    pub vcls: Option<Vec<crate::models::SchemasVclResponse>>,
    /// A list of Wordpress rules with this service.
    #[serde(rename = "wordpress", skip_serializing_if = "Option::is_none")]
    pub wordpress: Option<Vec<serde_json::Value>>,
}

impl ServiceVersionDetail {
    pub fn new() -> ServiceVersionDetail {
        ServiceVersionDetail {
            active: None,
            comment: None,
            deployed: None,
            locked: None,
            number: None,
            staging: None,
            testing: None,
            created_at: None,
            deleted_at: None,
            updated_at: None,
            service_id: None,
            backends: None,
            cache_settings: None,
            conditions: None,
            directors: None,
            domains: None,
            gzips: None,
            headers: None,
            healthchecks: None,
            request_settings: None,
            response_objects: None,
            settings: None,
            snippets: None,
            vcls: None,
            wordpress: None,
        }
    }
}


