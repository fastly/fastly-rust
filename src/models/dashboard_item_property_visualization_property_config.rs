/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// DashboardItemPropertyVisualizationPropertyConfig : [Configuration options](#visualization-config) for the given visualization.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DashboardItemPropertyVisualizationPropertyConfig {
    /// The type of chart to display. 
    #[serde(rename = "plot_type")]
    pub plot_type: PlotType,
    /// (Optional) The units to use to format the data. 
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
    /// (Optional) The aggregation function to apply to the dataset. 
    #[serde(rename = "calculation_method", skip_serializing_if = "Option::is_none")]
    pub calculation_method: Option<CalculationMethod>,
}

impl DashboardItemPropertyVisualizationPropertyConfig {
    /// [Configuration options](#visualization-config) for the given visualization.
    pub fn new(plot_type: PlotType) -> DashboardItemPropertyVisualizationPropertyConfig {
        DashboardItemPropertyVisualizationPropertyConfig {
            plot_type,
            format: None,
            calculation_method: None,
        }
    }
}

/// The type of chart to display. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlotType {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "single-metric")]
    SingleMetric,
    #[serde(rename = "donut")]
    Donut,
}

impl Default for PlotType {
    fn default() -> PlotType {
        Self::Line
    }
}
/// (Optional) The units to use to format the data. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "bytes")]
    Bytes,
    #[serde(rename = "percent")]
    Percent,
    #[serde(rename = "requests")]
    Requests,
    #[serde(rename = "responses")]
    Responses,
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "milliseconds")]
    Milliseconds,
    #[serde(rename = "ratio")]
    Ratio,
    #[serde(rename = "bitrate")]
    Bitrate,
}

impl Default for Format {
    fn default() -> Format {
        Self::Number
    }
}
/// (Optional) The aggregation function to apply to the dataset. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CalculationMethod {
    #[serde(rename = "avg")]
    Avg,
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "p95")]
    P95,
}

impl Default for CalculationMethod {
    fn default() -> CalculationMethod {
        Self::Avg
    }
}

