/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// DashboardItem : A dashboard item. Typically a data visualization like a chart.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DashboardItem {
    /// Dashboard item identifier (UUID)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A human-readable title for the dashboard item
    #[serde(rename = "title")]
    pub title: String,
    /// A human-readable subtitle for the dashboard item. Often a description of the visualization.
    #[serde(rename = "subtitle")]
    pub subtitle: String,
    #[serde(rename = "data_source")]
    pub data_source: Box<crate::models::DashboardItemPropertyDataSource>,
    #[serde(rename = "visualization")]
    pub visualization: Box<crate::models::DashboardItemPropertyVisualization>,
    /// The number of columns for the dashboard item to span. Dashboards are rendered on a 12-column grid on \"desktop\" screen sizes.
    #[serde(rename = "span", skip_serializing_if = "Option::is_none")]
    pub span: Option<i32>,
}

impl DashboardItem {
    /// A dashboard item. Typically a data visualization like a chart.
    pub fn new(title: String, subtitle: String, data_source: crate::models::DashboardItemPropertyDataSource, visualization: crate::models::DashboardItemPropertyVisualization) -> DashboardItem {
        DashboardItem {
            id: None,
            title,
            subtitle,
            data_source: Box::new(data_source),
            visualization: Box::new(visualization),
            span: None,
        }
    }
}


