/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectEndpointPolicy : The endpoints serving one network region. Replacing `endpoint_policies` preserves attachment timestamps for the same endpoint and region; reordering does not alter them, while changing preference updates the attachment's `updated_at` value.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectEndpointPolicy {
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<crate::models::NetworkProtectRegion>,
    /// The endpoints serving a policy region.
    #[serde(rename = "endpoints")]
    pub endpoints: Vec<crate::models::NetworkProtectPolicyEndpoint>,
}

impl NetworkProtectEndpointPolicy {
    /// The endpoints serving one network region. Replacing `endpoint_policies` preserves attachment timestamps for the same endpoint and region; reordering does not alter them, while changing preference updates the attachment's `updated_at` value.
    pub fn new(endpoints: Vec<crate::models::NetworkProtectPolicyEndpoint>) -> NetworkProtectEndpointPolicy {
        NetworkProtectEndpointPolicy {
            region: None,
            endpoints,
        }
    }
}


