/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_log_splunk`]
#[derive(Clone, Debug, Default)]
pub struct CreateLogSplunkParams {
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
    /// A secure certificate to authenticate a server with. Must be in PEM format.
    pub tls_ca_cert: Option<String>,
    /// The client certificate used to make authenticated requests. Must be in PEM format.
    pub tls_client_cert: Option<String>,
    /// The client private key used to make authenticated requests. Must be in PEM format.
    pub tls_client_key: Option<String>,
    /// The hostname to verify the server's certificate. This should be one of the Subject Alternative Name (SAN) fields for the certificate. Common Names (CN) are not supported.
    pub tls_hostname: Option<String>,
    /// The maximum number of logs sent in one request. Defaults `0` for unbounded.
    pub request_max_entries: Option<i32>,
    /// The maximum number of bytes sent in one request. Defaults `0` for unbounded.
    pub request_max_bytes: Option<i32>,
    /// The URL to post logs to.
    pub url: Option<String>,
    /// A Splunk token for use in posting logs over HTTP to your collector.
    pub token: Option<String>,
    pub use_tls: Option<crate::models::LoggingUseTls>
}

/// struct for passing parameters to the method [`delete_log_splunk`]
#[derive(Clone, Debug, Default)]
pub struct DeleteLogSplunkParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub logging_splunk_name: String
}

/// struct for passing parameters to the method [`get_log_splunk`]
#[derive(Clone, Debug, Default)]
pub struct GetLogSplunkParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub logging_splunk_name: String
}

/// struct for passing parameters to the method [`list_log_splunk`]
#[derive(Clone, Debug, Default)]
pub struct ListLogSplunkParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32
}

/// struct for passing parameters to the method [`update_log_splunk`]
#[derive(Clone, Debug, Default)]
pub struct UpdateLogSplunkParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub logging_splunk_name: String,
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
    /// A secure certificate to authenticate a server with. Must be in PEM format.
    pub tls_ca_cert: Option<String>,
    /// The client certificate used to make authenticated requests. Must be in PEM format.
    pub tls_client_cert: Option<String>,
    /// The client private key used to make authenticated requests. Must be in PEM format.
    pub tls_client_key: Option<String>,
    /// The hostname to verify the server's certificate. This should be one of the Subject Alternative Name (SAN) fields for the certificate. Common Names (CN) are not supported.
    pub tls_hostname: Option<String>,
    /// The maximum number of logs sent in one request. Defaults `0` for unbounded.
    pub request_max_entries: Option<i32>,
    /// The maximum number of bytes sent in one request. Defaults `0` for unbounded.
    pub request_max_bytes: Option<i32>,
    /// The URL to post logs to.
    pub url: Option<String>,
    /// A Splunk token for use in posting logs over HTTP to your collector.
    pub token: Option<String>,
    pub use_tls: Option<crate::models::LoggingUseTls>
}


/// struct for typed errors of method [`create_log_splunk`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogSplunkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_log_splunk`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLogSplunkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_log_splunk`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogSplunkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_log_splunk`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogSplunkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_log_splunk`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogSplunkError {
    UnknownValue(serde_json::Value),
}


/// Create a Splunk logging object for a particular service and version.
pub async fn create_log_splunk(configuration: &mut configuration::Configuration, params: CreateLogSplunkParams) -> Result<crate::models::LoggingSplunkResponse, Error<CreateLogSplunkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let name = params.name;
    let placement = params.placement;
    let format_version = params.format_version;
    let response_condition = params.response_condition;
    let format = params.format;
    let tls_ca_cert = params.tls_ca_cert;
    let tls_client_cert = params.tls_client_cert;
    let tls_client_key = params.tls_client_key;
    let tls_hostname = params.tls_hostname;
    let request_max_entries = params.request_max_entries;
    let request_max_bytes = params.request_max_bytes;
    let url = params.url;
    let token = params.token;
    let use_tls = params.use_tls;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/splunk", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
    if let Some(local_var_param_value) = tls_ca_cert {
        local_var_form_params.insert("tls_ca_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_client_cert {
        local_var_form_params.insert("tls_client_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_client_key {
        local_var_form_params.insert("tls_client_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_hostname {
        local_var_form_params.insert("tls_hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = request_max_entries {
        local_var_form_params.insert("request_max_entries", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = request_max_bytes {
        local_var_form_params.insert("request_max_bytes", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = url {
        local_var_form_params.insert("url", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = token {
        local_var_form_params.insert("token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = use_tls {
        local_var_form_params.insert("use_tls", local_var_param_value.to_string());
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
        let local_var_entity: Option<CreateLogSplunkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the Splunk logging object for a particular service and version.
pub async fn delete_log_splunk(configuration: &mut configuration::Configuration, params: DeleteLogSplunkParams) -> Result<crate::models::InlineResponse200, Error<DeleteLogSplunkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let logging_splunk_name = params.logging_splunk_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/splunk/{logging_splunk_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, logging_splunk_name=crate::apis::urlencode(logging_splunk_name));
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
        let local_var_entity: Option<DeleteLogSplunkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the details for a Splunk logging object for a particular service and version.
pub async fn get_log_splunk(configuration: &mut configuration::Configuration, params: GetLogSplunkParams) -> Result<crate::models::LoggingSplunkResponse, Error<GetLogSplunkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let logging_splunk_name = params.logging_splunk_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/splunk/{logging_splunk_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, logging_splunk_name=crate::apis::urlencode(logging_splunk_name));
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
        let local_var_entity: Option<GetLogSplunkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all of the Splunk logging objects for a particular service and version.
pub async fn list_log_splunk(configuration: &mut configuration::Configuration, params: ListLogSplunkParams) -> Result<Vec<crate::models::LoggingSplunkResponse>, Error<ListLogSplunkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/splunk", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
        let local_var_entity: Option<ListLogSplunkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the Splunk logging object for a particular service and version.
pub async fn update_log_splunk(configuration: &mut configuration::Configuration, params: UpdateLogSplunkParams) -> Result<crate::models::LoggingSplunkResponse, Error<UpdateLogSplunkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let logging_splunk_name = params.logging_splunk_name;
    let name = params.name;
    let placement = params.placement;
    let format_version = params.format_version;
    let response_condition = params.response_condition;
    let format = params.format;
    let tls_ca_cert = params.tls_ca_cert;
    let tls_client_cert = params.tls_client_cert;
    let tls_client_key = params.tls_client_key;
    let tls_hostname = params.tls_hostname;
    let request_max_entries = params.request_max_entries;
    let request_max_bytes = params.request_max_bytes;
    let url = params.url;
    let token = params.token;
    let use_tls = params.use_tls;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/splunk/{logging_splunk_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, logging_splunk_name=crate::apis::urlencode(logging_splunk_name));
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
    if let Some(local_var_param_value) = tls_ca_cert {
        local_var_form_params.insert("tls_ca_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_client_cert {
        local_var_form_params.insert("tls_client_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_client_key {
        local_var_form_params.insert("tls_client_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_hostname {
        local_var_form_params.insert("tls_hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = request_max_entries {
        local_var_form_params.insert("request_max_entries", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = request_max_bytes {
        local_var_form_params.insert("request_max_bytes", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = url {
        local_var_form_params.insert("url", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = token {
        local_var_form_params.insert("token", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = use_tls {
        local_var_form_params.insert("use_tls", local_var_param_value.to_string());
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
        let local_var_entity: Option<UpdateLogSplunkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

