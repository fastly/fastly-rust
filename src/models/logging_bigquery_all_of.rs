/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingBigqueryAllOf {
    /// The name of the BigQuery logging object. Used as a primary key for API access.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). Must produce JSON that matches the schema of your BigQuery table.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Your BigQuery dataset.
    #[serde(rename = "dataset", skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,
    /// Your BigQuery table.
    #[serde(rename = "table", skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    /// BigQuery table name suffix template. Optional.
    #[serde(rename = "template_suffix", skip_serializing_if = "Option::is_none")]
    pub template_suffix: Option<String>,
    /// Your Google Cloud Platform project ID. Required
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl LoggingBigqueryAllOf {
    pub fn new() -> LoggingBigqueryAllOf {
        LoggingBigqueryAllOf {
            name: None,
            format: None,
            dataset: None,
            table: None,
            template_suffix: None,
            project_id: None,
        }
    }
}


