/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_healthcheck`]
#[derive(Clone, Debug, Default)]
pub struct CreateHealthcheckParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// How often to run the health check in milliseconds.
    pub check_interval: Option<i32>,
    /// A freeform descriptive note.
    pub comment: Option<String>,
    /// The status code expected from the host.
    pub expected_response: Option<i32>,
    /// Array of custom headers that will be added to the health check probes.
    pub headers: Option<Vec<String>>,
    /// Which host to check.
    pub host: Option<String>,
    /// Whether to use version 1.0 or 1.1 HTTP.
    pub http_version: Option<String>,
    /// When loading a config, the initial number of probes to be seen as OK.
    pub initial: Option<i32>,
    /// Which HTTP method to use.
    pub method: Option<String>,
    /// The name of the health check.
    pub name: Option<String>,
    /// The path to check.
    pub path: Option<String>,
    /// How many health checks must succeed to be considered healthy.
    pub threshold: Option<i32>,
    /// Timeout in milliseconds.
    pub timeout: Option<i32>,
    /// The number of most recent health check queries to keep for this health check.
    pub window: Option<i32>
}

/// struct for passing parameters to the method [`delete_healthcheck`]
#[derive(Clone, Debug, Default)]
pub struct DeleteHealthcheckParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name of the health check.
    pub healthcheck_name: String
}

/// struct for passing parameters to the method [`get_healthcheck`]
#[derive(Clone, Debug, Default)]
pub struct GetHealthcheckParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name of the health check.
    pub healthcheck_name: String
}

/// struct for passing parameters to the method [`list_healthchecks`]
#[derive(Clone, Debug, Default)]
pub struct ListHealthchecksParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32
}

/// struct for passing parameters to the method [`update_healthcheck`]
#[derive(Clone, Debug, Default)]
pub struct UpdateHealthcheckParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name of the health check.
    pub healthcheck_name: String,
    /// How often to run the health check in milliseconds.
    pub check_interval: Option<i32>,
    /// A freeform descriptive note.
    pub comment: Option<String>,
    /// The status code expected from the host.
    pub expected_response: Option<i32>,
    /// Array of custom headers that will be added to the health check probes.
    pub headers: Option<Vec<String>>,
    /// Which host to check.
    pub host: Option<String>,
    /// Whether to use version 1.0 or 1.1 HTTP.
    pub http_version: Option<String>,
    /// When loading a config, the initial number of probes to be seen as OK.
    pub initial: Option<i32>,
    /// Which HTTP method to use.
    pub method: Option<String>,
    /// The name of the health check.
    pub name: Option<String>,
    /// The path to check.
    pub path: Option<String>,
    /// How many health checks must succeed to be considered healthy.
    pub threshold: Option<i32>,
    /// Timeout in milliseconds.
    pub timeout: Option<i32>,
    /// The number of most recent health check queries to keep for this health check.
    pub window: Option<i32>
}


/// struct for typed errors of method [`create_healthcheck`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateHealthcheckError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_healthcheck`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteHealthcheckError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_healthcheck`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHealthcheckError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_healthchecks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHealthchecksError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_healthcheck`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateHealthcheckError {
    UnknownValue(serde_json::Value),
}


/// Create a health check for a particular service and version.
pub async fn create_healthcheck(configuration: &mut configuration::Configuration, params: CreateHealthcheckParams) -> Result<crate::models::HealthcheckResponse, Error<CreateHealthcheckError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let check_interval = params.check_interval;
    let comment = params.comment;
    let expected_response = params.expected_response;
    let headers = params.headers;
    let host = params.host;
    let http_version = params.http_version;
    let initial = params.initial;
    let method = params.method;
    let name = params.name;
    let path = params.path;
    let threshold = params.threshold;
    let timeout = params.timeout;
    let window = params.window;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/healthcheck", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
    if let Some(local_var_param_value) = check_interval {
        local_var_form_params.insert("check_interval", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comment {
        local_var_form_params.insert("comment", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = expected_response {
        local_var_form_params.insert("expected_response", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = headers {
        local_var_form_params.insert("headers", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    if let Some(local_var_param_value) = host {
        local_var_form_params.insert("host", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = http_version {
        local_var_form_params.insert("http_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = initial {
        local_var_form_params.insert("initial", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = method {
        local_var_form_params.insert("method", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = path {
        local_var_form_params.insert("path", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = threshold {
        local_var_form_params.insert("threshold", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = timeout {
        local_var_form_params.insert("timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = window {
        local_var_form_params.insert("window", local_var_param_value.to_string());
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
        let local_var_entity: Option<CreateHealthcheckError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the health check for a particular service and version.
pub async fn delete_healthcheck(configuration: &mut configuration::Configuration, params: DeleteHealthcheckParams) -> Result<crate::models::InlineResponse200, Error<DeleteHealthcheckError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let healthcheck_name = params.healthcheck_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/healthcheck/{healthcheck_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, healthcheck_name=crate::apis::urlencode(healthcheck_name));
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
        let local_var_entity: Option<DeleteHealthcheckError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the health check for a particular service and version.
pub async fn get_healthcheck(configuration: &mut configuration::Configuration, params: GetHealthcheckParams) -> Result<crate::models::HealthcheckResponse, Error<GetHealthcheckError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let healthcheck_name = params.healthcheck_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/healthcheck/{healthcheck_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, healthcheck_name=crate::apis::urlencode(healthcheck_name));
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
        let local_var_entity: Option<GetHealthcheckError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all of the health checks for a particular service and version.
pub async fn list_healthchecks(configuration: &mut configuration::Configuration, params: ListHealthchecksParams) -> Result<Vec<crate::models::HealthcheckResponse>, Error<ListHealthchecksError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/healthcheck", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
        let local_var_entity: Option<ListHealthchecksError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the health check for a particular service and version.
pub async fn update_healthcheck(configuration: &mut configuration::Configuration, params: UpdateHealthcheckParams) -> Result<crate::models::HealthcheckResponse, Error<UpdateHealthcheckError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let healthcheck_name = params.healthcheck_name;
    let check_interval = params.check_interval;
    let comment = params.comment;
    let expected_response = params.expected_response;
    let headers = params.headers;
    let host = params.host;
    let http_version = params.http_version;
    let initial = params.initial;
    let method = params.method;
    let name = params.name;
    let path = params.path;
    let threshold = params.threshold;
    let timeout = params.timeout;
    let window = params.window;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/healthcheck/{healthcheck_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, healthcheck_name=crate::apis::urlencode(healthcheck_name));
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
    if let Some(local_var_param_value) = check_interval {
        local_var_form_params.insert("check_interval", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comment {
        local_var_form_params.insert("comment", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = expected_response {
        local_var_form_params.insert("expected_response", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = headers {
        local_var_form_params.insert("headers", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    if let Some(local_var_param_value) = host {
        local_var_form_params.insert("host", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = http_version {
        local_var_form_params.insert("http_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = initial {
        local_var_form_params.insert("initial", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = method {
        local_var_form_params.insert("method", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = path {
        local_var_form_params.insert("path", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = threshold {
        local_var_form_params.insert("threshold", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = timeout {
        local_var_form_params.insert("timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = window {
        local_var_form_params.insert("window", local_var_param_value.to_string());
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
        let local_var_entity: Option<UpdateHealthcheckError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

