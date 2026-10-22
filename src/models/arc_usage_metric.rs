/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcUsageMetric : A usage metric measured for a single date (or hour, when `granularity` is `hourly`).



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcUsageMetric {
    /// The date (or hour) the usage was measured. Formatted as `YYYY-MM-DD` when `granularity` is `daily`, or `YYYY-MM-DD HH:00:00` when `granularity` is `hourly`. 
    #[serde(rename = "date")]
    pub date: String,
    /// The type of usage being measured.
    #[serde(rename = "usage_type")]
    pub usage_type: UsageType,
    /// The quantity of the usage type for this date.
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// The ID of the virtual key this usage is attributed to.
    #[serde(rename = "virtual_key_id", skip_serializing_if = "Option::is_none")]
    pub virtual_key_id: Option<String>,
    /// The display name of the virtual key this usage is attributed to.
    #[serde(rename = "virtual_key_name", skip_serializing_if = "Option::is_none")]
    pub virtual_key_name: Option<String>,
    /// The ID of the provider connection originally requested, before any failover occurred.
    #[serde(rename = "primary_provider", skip_serializing_if = "Option::is_none")]
    pub primary_provider: Option<String>,
    /// The name of the provider connection that ultimately served the request.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// The ID of the provider connection that ultimately served the request.
    #[serde(rename = "provider_connection_id", skip_serializing_if = "Option::is_none")]
    pub provider_connection_id: Option<String>,
    /// The AI model identifier used.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl ArcUsageMetric {
    /// A usage metric measured for a single date (or hour, when `granularity` is `hourly`).
    pub fn new(date: String, usage_type: UsageType, quantity: i32) -> ArcUsageMetric {
        ArcUsageMetric {
            date,
            usage_type,
            quantity,
            virtual_key_id: None,
            virtual_key_name: None,
            primary_provider: None,
            provider: None,
            provider_connection_id: None,
            model: None,
        }
    }
}

/// The type of usage being measured.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UsageType {
    #[serde(rename = "requests")]
    Requests,
    #[serde(rename = "sessions")]
    Sessions,
    #[serde(rename = "input_tokens")]
    InputTokens,
    #[serde(rename = "output_tokens")]
    OutputTokens,
}

impl Default for UsageType {
    fn default() -> UsageType {
        Self::Requests
    }
}

