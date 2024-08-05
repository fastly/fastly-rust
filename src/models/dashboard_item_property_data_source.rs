/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// DashboardItemPropertyDataSource : [An object](#data-source) which describes the data to display.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DashboardItemPropertyDataSource {
    /// The source of the data to display.
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "config")]
    pub config: Box<crate::models::DashboardItemPropertyDataSourcePropertyConfig>,
}

impl DashboardItemPropertyDataSource {
    /// [An object](#data-source) which describes the data to display.
    pub fn new(_type: Type, config: crate::models::DashboardItemPropertyDataSourcePropertyConfig) -> DashboardItemPropertyDataSource {
        DashboardItemPropertyDataSource {
            _type,
            config: Box::new(config),
        }
    }
}

/// The source of the data to display.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "stats.edge")]
    Edge,
    #[serde(rename = "stats.domain")]
    Domain,
    #[serde(rename = "stats.origin")]
    Origin,
}

impl Default for Type {
    fn default() -> Type {
        Self::Edge
    }
}

