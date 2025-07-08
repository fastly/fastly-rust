/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingGrafanacloudlogsAdditional {
    /// A Fastly [log format string](https://www.fastly.com/documentation/guides/integrations/streaming-logs/custom-log-formats/).
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The Grafana Cloud Logs Dataset you want to log to.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// The URL of the Loki instance in your Grafana stack.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The Grafana Access Policy token with `logs:write` access scoped to your Loki instance.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// The Stream Labels, a JSON string used to identify the stream.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
}

impl LoggingGrafanacloudlogsAdditional {
    pub fn new() -> LoggingGrafanacloudlogsAdditional {
        LoggingGrafanacloudlogsAdditional {
            format: None,
            user: None,
            url: None,
            token: None,
            index: None,
        }
    }
}


