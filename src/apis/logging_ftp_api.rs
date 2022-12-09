/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_log_ftp`]
#[derive(Clone, Debug, Default)]
pub struct CreateLogFtpParams {
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
    /// An hostname or IPv4 address.
    pub address: Option<String>,
    /// Hostname used.
    pub hostname: Option<String>,
    /// IPv4 address of the host.
    pub ipv4: Option<String>,
    /// The password for the server. For anonymous use an email address.
    pub password: Option<String>,
    /// The path to upload log files to. If the path ends in `/` then it is treated as a directory.
    pub path: Option<String>,
    /// The port number.
    pub port: Option<i32>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    pub public_key: Option<String>,
    /// The username for the server. Can be anonymous.
    pub user: Option<String>
}

/// struct for passing parameters to the method [`delete_log_ftp`]
#[derive(Clone, Debug, Default)]
pub struct DeleteLogFtpParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub logging_ftp_name: String
}

/// struct for passing parameters to the method [`get_log_ftp`]
#[derive(Clone, Debug, Default)]
pub struct GetLogFtpParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub logging_ftp_name: String
}

/// struct for passing parameters to the method [`list_log_ftp`]
#[derive(Clone, Debug, Default)]
pub struct ListLogFtpParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32
}

/// struct for passing parameters to the method [`update_log_ftp`]
#[derive(Clone, Debug, Default)]
pub struct UpdateLogFtpParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub logging_ftp_name: String,
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
    /// An hostname or IPv4 address.
    pub address: Option<String>,
    /// Hostname used.
    pub hostname: Option<String>,
    /// IPv4 address of the host.
    pub ipv4: Option<String>,
    /// The password for the server. For anonymous use an email address.
    pub password: Option<String>,
    /// The path to upload log files to. If the path ends in `/` then it is treated as a directory.
    pub path: Option<String>,
    /// The port number.
    pub port: Option<i32>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    pub public_key: Option<String>,
    /// The username for the server. Can be anonymous.
    pub user: Option<String>
}


/// struct for typed errors of method [`create_log_ftp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogFtpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_log_ftp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLogFtpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_log_ftp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogFtpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_log_ftp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogFtpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_log_ftp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogFtpError {
    UnknownValue(serde_json::Value),
}


/// Create a FTP for a particular service and version.
pub async fn create_log_ftp(configuration: &mut configuration::Configuration, params: CreateLogFtpParams) -> Result<crate::models::LoggingFtpResponse, Error<CreateLogFtpError>> {
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
    let address = params.address;
    let hostname = params.hostname;
    let ipv4 = params.ipv4;
    let password = params.password;
    let path = params.path;
    let port = params.port;
    let public_key = params.public_key;
    let user = params.user;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/ftp", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
    if let Some(local_var_param_value) = address {
        local_var_form_params.insert("address", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = hostname {
        local_var_form_params.insert("hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ipv4 {
        local_var_form_params.insert("ipv4", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = password {
        local_var_form_params.insert("password", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = path {
        local_var_form_params.insert("path", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = port {
        local_var_form_params.insert("port", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = public_key {
        local_var_form_params.insert("public_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = user {
        local_var_form_params.insert("user", local_var_param_value.to_string());
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
        let local_var_entity: Option<CreateLogFtpError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the FTP for a particular service and version.
pub async fn delete_log_ftp(configuration: &mut configuration::Configuration, params: DeleteLogFtpParams) -> Result<crate::models::InlineResponse200, Error<DeleteLogFtpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let logging_ftp_name = params.logging_ftp_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/ftp/{logging_ftp_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, logging_ftp_name=crate::apis::urlencode(logging_ftp_name));
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
        let local_var_entity: Option<DeleteLogFtpError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the FTP for a particular service and version.
pub async fn get_log_ftp(configuration: &mut configuration::Configuration, params: GetLogFtpParams) -> Result<crate::models::LoggingFtpResponse, Error<GetLogFtpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let logging_ftp_name = params.logging_ftp_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/ftp/{logging_ftp_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, logging_ftp_name=crate::apis::urlencode(logging_ftp_name));
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
        let local_var_entity: Option<GetLogFtpError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all of the FTPs for a particular service and version.
pub async fn list_log_ftp(configuration: &mut configuration::Configuration, params: ListLogFtpParams) -> Result<Vec<crate::models::LoggingFtpResponse>, Error<ListLogFtpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/ftp", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
        let local_var_entity: Option<ListLogFtpError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the FTP for a particular service and version.
pub async fn update_log_ftp(configuration: &mut configuration::Configuration, params: UpdateLogFtpParams) -> Result<crate::models::LoggingFtpResponse, Error<UpdateLogFtpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let logging_ftp_name = params.logging_ftp_name;
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
    let address = params.address;
    let hostname = params.hostname;
    let ipv4 = params.ipv4;
    let password = params.password;
    let path = params.path;
    let port = params.port;
    let public_key = params.public_key;
    let user = params.user;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/ftp/{logging_ftp_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, logging_ftp_name=crate::apis::urlencode(logging_ftp_name));
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
    if let Some(local_var_param_value) = address {
        local_var_form_params.insert("address", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = hostname {
        local_var_form_params.insert("hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ipv4 {
        local_var_form_params.insert("ipv4", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = password {
        local_var_form_params.insert("password", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = path {
        local_var_form_params.insert("path", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = port {
        local_var_form_params.insert("port", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = public_key {
        local_var_form_params.insert("public_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = user {
        local_var_form_params.insert("user", local_var_param_value.to_string());
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
        let local_var_entity: Option<UpdateLogFtpError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

