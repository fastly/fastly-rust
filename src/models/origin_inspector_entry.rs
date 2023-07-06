/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OriginInspectorEntry {
    #[serde(rename = "dimensions", skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Box<crate::models::OriginInspectorDimensions>>,
    /// An array of values representing the metric values at each point in time. Note that this dataset is sparse: only the keys with non-zero values will be included in the record. 
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<crate::models::OriginInspectorValues>>,
}

impl OriginInspectorEntry {
    pub fn new() -> OriginInspectorEntry {
        OriginInspectorEntry {
            dimensions: None,
            values: None,
        }
    }
}


