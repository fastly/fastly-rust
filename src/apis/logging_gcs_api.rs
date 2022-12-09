/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_log_gcs`]
#[derive(Clone, Debug, Default)]
pub struct CreateLogGcsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub name: Option<String>,
    /// Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`. 
    pub placement: Option<String>,
    /// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
    pub format_version: Option<i32>,
    /// The name of an existing condition in the configured endpoint, or leave blank to always execute.
    pub response_condition: Option<String>,
    /// A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats).
    pub format: Option<String>,
    /// How the message should be formatted.
    pub message_type: Option<String>,
    /// A timestamp format
    pub timestamp_format: Option<String>,
    /// How frequently log files are finalized so they can be available for reading (in seconds).
    pub period: Option<i32>,
    /// The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    pub gzip_level: Option<i32>,
    /// The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    pub compression_codec: Option<String>,
    /// Your Google Cloud Platform service account email address. The `client_email` field in your service account authentication JSON. Not required if `account_name` is specified.
    pub user: Option<String>,
    /// Your Google Cloud Platform account secret key. The `private_key` field in your service account authentication JSON. Not required if `account_name` is specified.
    pub secret_key: Option<String>,
    /// The name of the Google Cloud Platform service account associated with the target log collection service. Not required if `user` and `secret_key` are provided.
    pub account_name: Option<String>,
    /// The name of the GCS bucket.
    pub bucket_name: Option<String>,
    /// The path to upload logs to.
    pub path: Option<String>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    pub public_key: Option<String>,
    /// Your Google Cloud Platform project ID. Required
    pub project_id: Option<String>
}

/// struct for passing parameters to the method [`delete_log_gcs`]
#[derive(Clone, Debug, Default)]
pub struct DeleteLogGcsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub logging_gcs_name: String
}

/// struct for passing parameters to the method [`get_log_gcs`]
#[derive(Clone, Debug, Default)]
pub struct GetLogGcsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub logging_gcs_name: String
}

/// struct for passing parameters to the method [`list_log_gcs`]
#[derive(Clone, Debug, Default)]
pub struct ListLogGcsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32
}

/// struct for passing parameters to the method [`update_log_gcs`]
#[derive(Clone, Debug, Default)]
pub struct UpdateLogGcsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub logging_gcs_name: String,
    /// The name for the real-time logging configuration.
    pub name: Option<String>,
    /// Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`. 
    pub placement: Option<String>,
    /// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
    pub format_version: Option<i32>,
    /// The name of an existing condition in the configured endpoint, or leave blank to always execute.
    pub response_condition: Option<String>,
    /// A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats).
    pub format: Option<String>,
    /// How the message should be formatted.
    pub message_type: Option<String>,
    /// A timestamp format
    pub timestamp_format: Option<String>,
    /// How frequently log files are finalized so they can be available for reading (in seconds).
    pub period: Option<i32>,
    /// The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    pub gzip_level: Option<i32>,
    /// The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    pub compression_codec: Option<String>,
    /// Your Google Cloud Platform service account email address. The `client_email` field in your service account authentication JSON. Not required if `account_name` is specified.
    pub user: Option<String>,
    /// Your Google Cloud Platform account secret key. The `private_key` field in your service account authentication JSON. Not required if `account_name` is specified.
    pub secret_key: Option<String>,
    /// The name of the Google Cloud Platform service account associated with the target log collection service. Not required if `user` and `secret_key` are provided.
    pub account_name: Option<String>,
    /// The name of the GCS bucket.
    pub bucket_name: Option<String>,
    /// The path to upload logs to.
    pub path: Option<String>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    pub public_key: Option<String>,
    /// Your Google Cloud Platform project ID. Required
    pub project_id: Option<String>
}


/// struct for typed errors of method [`create_log_gcs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogGcsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_log_gcs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLogGcsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_log_gcs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogGcsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_log_gcs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogGcsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_log_gcs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogGcsError {
    UnknownValue(serde_json::Value),
}


/// Create GCS logging for a particular service and version.
pub async fn create_log_gcs(configuration: &mut configuration::Configuration, params: CreateLogGcsParams) -> Result<crate::models::LoggingGcsResponse, Error<CreateLogGcsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let name = params.name;
    let placement = params.placement;
    let format_version = params.format_version;
    let response_condition = params.response_condition;
    let format = params.format;
    let message_type = params.message_type;
    let timestamp_format = params.timestamp_format;
    let period = params.period;
    let gzip_level = params.gzip_level;
    let compression_codec = params.compression_codec;
    let user = params.user;
    let secret_key = params.secret_key;
    let account_name = params.account_name;
    let bucket_name = params.bucket_name;
    let path = params.path;
    let public_key = params.public_key;
    let project_id = params.project_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/gcs", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Fastly-Key", local_var_value);
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = placement {
        local_var_form_params.insert("placement", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = format_version {
        local_var_form_params.insert("format_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = response_condition {
        local_var_form_params.insert("response_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = format {
        local_var_form_params.insert("format", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = message_type {
        local_var_form_params.insert("message_type", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = timestamp_format {
        local_var_form_params.insert("timestamp_format", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = period {
        local_var_form_params.insert("period", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = gzip_level {
        local_var_form_params.insert("gzip_level", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = compression_codec {
        local_var_form_params.insert("compression_codec", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = user {
        local_var_form_params.insert("user", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = secret_key {
        local_var_form_params.insert("secret_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = account_name {
        local_var_form_params.insert("account_name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = bucket_name {
        local_var_form_params.insert("bucket_name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = path {
        local_var_form_params.insert("path", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = public_key {
        local_var_form_params.insert("public_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = project_id {
        local_var_form_params.insert("project_id", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "POST" != "GET" && "POST" != "HEAD" {
      let headers = local_var_resp.headers();
      local_var_configuration.rate_limit_remaining = match headers.get("Fastly-RateLimit-Remaining") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => configuration::DEFAULT_RATELIMIT,
      };
      local_var_configuration.rate_limit_reset = match headers.get("Fastly-RateLimit-Reset") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => 0,
      };
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateLogGcsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the GCS Logging for a particular service and version.
pub async fn delete_log_gcs(configuration: &mut configuration::Configuration, params: DeleteLogGcsParams) -> Result<crate::models::InlineResponse200, Error<DeleteLogGcsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let logging_gcs_name = params.logging_gcs_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/gcs/{logging_gcs_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, logging_gcs_name=crate::apis::urlencode(logging_gcs_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Fastly-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "DELETE" != "GET" && "DELETE" != "HEAD" {
      let headers = local_var_resp.headers();
      local_var_configuration.rate_limit_remaining = match headers.get("Fastly-RateLimit-Remaining") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => configuration::DEFAULT_RATELIMIT,
      };
      local_var_configuration.rate_limit_reset = match headers.get("Fastly-RateLimit-Reset") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => 0,
      };
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteLogGcsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the GCS Logging for a particular service and version.
pub async fn get_log_gcs(configuration: &mut configuration::Configuration, params: GetLogGcsParams) -> Result<crate::models::LoggingGcsResponse, Error<GetLogGcsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let logging_gcs_name = params.logging_gcs_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/gcs/{logging_gcs_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, logging_gcs_name=crate::apis::urlencode(logging_gcs_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Fastly-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "GET" != "GET" && "GET" != "HEAD" {
      let headers = local_var_resp.headers();
      local_var_configuration.rate_limit_remaining = match headers.get("Fastly-RateLimit-Remaining") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => configuration::DEFAULT_RATELIMIT,
      };
      local_var_configuration.rate_limit_reset = match headers.get("Fastly-RateLimit-Reset") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => 0,
      };
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLogGcsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all of the GCS log endpoints for a particular service and version.
pub async fn list_log_gcs(configuration: &mut configuration::Configuration, params: ListLogGcsParams) -> Result<Vec<crate::models::LoggingGcsResponse>, Error<ListLogGcsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/gcs", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Fastly-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "GET" != "GET" && "GET" != "HEAD" {
      let headers = local_var_resp.headers();
      local_var_configuration.rate_limit_remaining = match headers.get("Fastly-RateLimit-Remaining") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => configuration::DEFAULT_RATELIMIT,
      };
      local_var_configuration.rate_limit_reset = match headers.get("Fastly-RateLimit-Reset") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => 0,
      };
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListLogGcsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the GCS for a particular service and version.
pub async fn update_log_gcs(configuration: &mut configuration::Configuration, params: UpdateLogGcsParams) -> Result<crate::models::LoggingGcsResponse, Error<UpdateLogGcsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let logging_gcs_name = params.logging_gcs_name;
    let name = params.name;
    let placement = params.placement;
    let format_version = params.format_version;
    let response_condition = params.response_condition;
    let format = params.format;
    let message_type = params.message_type;
    let timestamp_format = params.timestamp_format;
    let period = params.period;
    let gzip_level = params.gzip_level;
    let compression_codec = params.compression_codec;
    let user = params.user;
    let secret_key = params.secret_key;
    let account_name = params.account_name;
    let bucket_name = params.bucket_name;
    let path = params.path;
    let public_key = params.public_key;
    let project_id = params.project_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/gcs/{logging_gcs_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, logging_gcs_name=crate::apis::urlencode(logging_gcs_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Fastly-Key", local_var_value);
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = placement {
        local_var_form_params.insert("placement", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = format_version {
        local_var_form_params.insert("format_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = response_condition {
        local_var_form_params.insert("response_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = format {
        local_var_form_params.insert("format", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = message_type {
        local_var_form_params.insert("message_type", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = timestamp_format {
        local_var_form_params.insert("timestamp_format", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = period {
        local_var_form_params.insert("period", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = gzip_level {
        local_var_form_params.insert("gzip_level", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = compression_codec {
        local_var_form_params.insert("compression_codec", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = user {
        local_var_form_params.insert("user", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = secret_key {
        local_var_form_params.insert("secret_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = account_name {
        local_var_form_params.insert("account_name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = bucket_name {
        local_var_form_params.insert("bucket_name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = path {
        local_var_form_params.insert("path", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = public_key {
        local_var_form_params.insert("public_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = project_id {
        local_var_form_params.insert("project_id", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "PUT" != "GET" && "PUT" != "HEAD" {
      let headers = local_var_resp.headers();
      local_var_configuration.rate_limit_remaining = match headers.get("Fastly-RateLimit-Remaining") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => configuration::DEFAULT_RATELIMIT,
      };
      local_var_configuration.rate_limit_reset = match headers.get("Fastly-RateLimit-Reset") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => 0,
      };
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateLogGcsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

