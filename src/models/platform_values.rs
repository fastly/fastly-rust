/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// PlatformValues : The results of the query, optionally filtered and grouped over the requested timespan.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PlatformValues {
    /// Timestamp of the metrics data point.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 25th percentile of time to first byte from origin, in microseconds.
    #[serde(rename = "ttfb_origin_p25_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_origin_p25_us: Option<f32>,
    /// 50th percentile of time to first byte from origin, in microseconds.
    #[serde(rename = "ttfb_origin_p50_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_origin_p50_us: Option<f32>,
    /// 75th percentile of time to first byte from origin, in microseconds.
    #[serde(rename = "ttfb_origin_p75_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_origin_p75_us: Option<f32>,
    /// 95th percentile of time to first byte from origin, in microseconds.
    #[serde(rename = "ttfb_origin_p95_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_origin_p95_us: Option<f32>,
    /// 99th percentile of time to first byte from origin, in microseconds.
    #[serde(rename = "ttfb_origin_p99_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_origin_p99_us: Option<f32>,
    /// 25th percentile of time to first byte from shield, in microseconds.
    #[serde(rename = "ttfb_shield_p25_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_shield_p25_us: Option<f32>,
    /// 50th percentile of time to first byte from shield, in microseconds.
    #[serde(rename = "ttfb_shield_p50_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_shield_p50_us: Option<f32>,
    /// 75th percentile of time to first byte from shield, in microseconds.
    #[serde(rename = "ttfb_shield_p75_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_shield_p75_us: Option<f32>,
    /// 95th percentile of time to first byte from shield, in microseconds.
    #[serde(rename = "ttfb_shield_p95_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_shield_p95_us: Option<f32>,
    /// 99th percentile of time to first byte from shield, in microseconds.
    #[serde(rename = "ttfb_shield_p99_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_shield_p99_us: Option<f32>,
    /// 25th percentile of time to first byte from edge, in microseconds.
    #[serde(rename = "ttfb_edge_p25_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_edge_p25_us: Option<f32>,
    /// 50th percentile of time to first byte from edge, in microseconds.
    #[serde(rename = "ttfb_edge_p50_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_edge_p50_us: Option<f32>,
    /// 75th percentile of time to first byte from edge, in microseconds.
    #[serde(rename = "ttfb_edge_p75_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_edge_p75_us: Option<f32>,
    /// 95th percentile of time to first byte from edge, in microseconds.
    #[serde(rename = "ttfb_edge_p95_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_edge_p95_us: Option<f32>,
    /// 99th percentile of time to first byte from edge, in microseconds.
    #[serde(rename = "ttfb_edge_p99_us", skip_serializing_if = "Option::is_none")]
    pub ttfb_edge_p99_us: Option<f32>,
}

impl PlatformValues {
    /// The results of the query, optionally filtered and grouped over the requested timespan.
    pub fn new() -> PlatformValues {
        PlatformValues {
            timestamp: None,
            ttfb_origin_p25_us: None,
            ttfb_origin_p50_us: None,
            ttfb_origin_p75_us: None,
            ttfb_origin_p95_us: None,
            ttfb_origin_p99_us: None,
            ttfb_shield_p25_us: None,
            ttfb_shield_p50_us: None,
            ttfb_shield_p75_us: None,
            ttfb_shield_p95_us: None,
            ttfb_shield_p99_us: None,
            ttfb_edge_p25_us: None,
            ttfb_edge_p50_us: None,
            ttfb_edge_p75_us: None,
            ttfb_edge_p95_us: None,
            ttfb_edge_p99_us: None,
        }
    }
}


