/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcFailoverTarget : A provider connection to retry a request against.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcFailoverTarget {
    /// The ID of the provider connection to retry against.
    #[serde(rename = "provider_connection_id")]
    pub provider_connection_id: String,
}

impl ArcFailoverTarget {
    /// A provider connection to retry a request against.
    pub fn new(provider_connection_id: String) -> ArcFailoverTarget {
        ArcFailoverTarget {
            provider_connection_id,
        }
    }
}


