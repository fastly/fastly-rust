/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// DashboardItemPropertyDataSourcePropertyConfig : [Configuration options](#data-source-config) for the selected data source.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DashboardItemPropertyDataSourcePropertyConfig {
    /// The metrics to visualize. Valid options are defined by the selected [data source](#field_data_source).
    #[serde(rename = "metrics")]
    pub metrics: Vec<String>,
}

impl DashboardItemPropertyDataSourcePropertyConfig {
    /// [Configuration options](#data-source-config) for the selected data source.
    pub fn new(metrics: Vec<String>) -> DashboardItemPropertyDataSourcePropertyConfig {
        DashboardItemPropertyDataSourcePropertyConfig {
            metrics,
        }
    }
}


