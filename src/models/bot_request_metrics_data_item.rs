/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// BotRequestMetricsDataItem : A time series data item containing bot request metric counts. The `bot` field is present when grouped by bot. The `datacenter` field is present when grouped by data center. This dataset is sparse, meaning only keys with non-zero values are included. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotRequestMetricsDataItem {
    /// Timestamp for the time series being reported.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Bot identifier for which data was recorded. Present only when grouped by bot.
    #[serde(rename = "bot", skip_serializing_if = "Option::is_none")]
    pub bot: Option<String>,
    /// Datacenter (POP) for which data was recorded. Present only when grouped by datacenter.
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    /// Count of edge requests where a bot was detected and the request was allowed by the configured policy.
    #[serde(rename = "bot_edge_requests_allowed_count", skip_serializing_if = "Option::is_none")]
    pub bot_edge_requests_allowed_count: Option<i64>,
    /// Count of edge requests where a bot was detected and a client challenge was issued for the request by the configured policy.
    #[serde(rename = "bot_edge_requests_challenged_count", skip_serializing_if = "Option::is_none")]
    pub bot_edge_requests_challenged_count: Option<i64>,
    /// Count of edge requests where a bot was detected and the request was blocked by the configured policy.
    #[serde(rename = "bot_edge_requests_blocked_count", skip_serializing_if = "Option::is_none")]
    pub bot_edge_requests_blocked_count: Option<i64>,
    /// Count of edge requests where a bot was detected and the client making the request has already solved a challenge issued by the configured policy.
    #[serde(rename = "bot_edge_requests_challenge_token_valid_count", skip_serializing_if = "Option::is_none")]
    pub bot_edge_requests_challenge_token_valid_count: Option<i64>,
}

impl BotRequestMetricsDataItem {
    /// A time series data item containing bot request metric counts. The `bot` field is present when grouped by bot. The `datacenter` field is present when grouped by data center. This dataset is sparse, meaning only keys with non-zero values are included. 
    pub fn new() -> BotRequestMetricsDataItem {
        BotRequestMetricsDataItem {
            timestamp: None,
            bot: None,
            datacenter: None,
            bot_edge_requests_allowed_count: None,
            bot_edge_requests_challenged_count: None,
            bot_edge_requests_blocked_count: None,
            bot_edge_requests_challenge_token_valid_count: None,
        }
    }
}


