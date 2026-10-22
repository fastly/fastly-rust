/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcModel : An AI model available through AI Runtime Control.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcModel {
    /// The model identifier.
    #[serde(rename = "id")]
    pub id: String,
    /// The human-readable name of the model.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// The ID of the provider this model belongs to.
    #[serde(rename = "provider_id")]
    pub provider_id: String,
}

impl ArcModel {
    /// An AI model available through AI Runtime Control.
    pub fn new(id: String, display_name: String, provider_id: String) -> ArcModel {
        ArcModel {
            id,
            display_name,
            provider_id,
        }
    }
}


