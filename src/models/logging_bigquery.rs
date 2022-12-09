/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingBigquery {
    /// The name of the BigQuery logging object. Used as a primary key for API access.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`. 
    #[serde(rename = "placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
    /// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
    #[serde(rename = "format_version", skip_serializing_if = "Option::is_none")]
    pub format_version: Option<FormatVersion>,
    /// The name of an existing condition in the configured endpoint, or leave blank to always execute.
    #[serde(rename = "response_condition", skip_serializing_if = "Option::is_none")]
    pub response_condition: Option<String>,
    /// A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). Must produce JSON that matches the schema of your BigQuery table.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Your Google Cloud Platform service account email address. The `client_email` field in your service account authentication JSON. Not required if `account_name` is specified.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Your Google Cloud Platform account secret key. The `private_key` field in your service account authentication JSON. Not required if `account_name` is specified.
    #[serde(rename = "secret_key", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// The name of the Google Cloud Platform service account associated with the target log collection service. Not required if `user` and `secret_key` are provided.
    #[serde(rename = "account_name", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
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

impl LoggingBigquery {
    pub fn new() -> LoggingBigquery {
        LoggingBigquery {
            name: None,
            placement: None,
            format_version: None,
            response_condition: None,
            format: None,
            user: None,
            secret_key: None,
            account_name: None,
            dataset: None,
            table: None,
            template_suffix: None,
            project_id: None,
        }
    }
}

/// Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Placement {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "waf_debug")]
    WafDebug,
    #[serde(rename = "null")]
    Null,
}

impl Default for Placement {
    fn default() -> Placement {
        Self::None
    }
}
/// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormatVersion {
    #[serde(rename = "1")]
    FormatVersionV1,
    #[serde(rename = "2")]
    FormatVersionV2,
}

impl Default for FormatVersion {
    fn default() -> FormatVersion {
        Self::FormatVersionV1
    }
}

