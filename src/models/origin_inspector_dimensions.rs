/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// OriginInspectorDimensions : The unique combination of dimensions associated with this timeseries.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OriginInspectorDimensions {
    /// The geographic region from which the edge responses in this data entry were delivered. If unspecified, results are aggregated across regions.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The POP from which the edge responses in this data entry were delivered. If unspecified, results are aggregated across POPs.
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    /// The origin host from which the edge responses in this data entry were delivered. If unspecified, results are aggregated across origin hosts.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl OriginInspectorDimensions {
    /// The unique combination of dimensions associated with this timeseries.
    pub fn new() -> OriginInspectorDimensions {
        OriginInspectorDimensions {
            region: None,
            datacenter: None,
            host: None,
        }
    }
}


