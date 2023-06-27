/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_rate_limiter`]
#[derive(Clone, Debug, Default)]
pub struct CreateRateLimiterParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// A human readable name for the rate limiting rule.
    pub name: Option<String>,
    /// The name of an Edge Dictionary containing URIs as keys. If not defined or `null`, all origin URIs will be rate limited.
    pub uri_dictionary_name: Option<String>,
    /// Array of HTTP methods to apply rate limiting to.
    pub http_methods: Option<Vec<String>>,
    /// Upper limit of requests per second allowed by the rate limiter.
    pub rps_limit: Option<i32>,
    /// Number of seconds during which the RPS limit must be exceeded in order to trigger a violation.
    pub window_size: Option<i32>,
    /// Array of VCL variables used to generate a counter key to identify a client. Example variables include `req.http.Fastly-Client-IP`, `req.http.User-Agent`, or a custom header like `req.http.API-Key`.
    pub client_key: Option<Vec<String>>,
    /// Length of time in minutes that the rate limiter is in effect after the initial violation is detected.
    pub penalty_box_duration: Option<i32>,
    /// The action to take when a rate limiter violation is detected.
    pub action: Option<String>,
    /// Name of existing response object. Required if `action` is `response_object`. Note that the rate limiter response is only updated to reflect the response object content when saving the rate limiter configuration.
    pub response_object_name: Option<String>,
    /// Name of the type of logging endpoint to be used when action is `log_only`. The logging endpoint type is used to determine the appropriate log format to use when emitting log entries.
    pub logger_type: Option<String>,
    /// Revision number of the rate limiting feature implementation. Defaults to the most recent revision.
    pub feature_revision: Option<i32>
}

/// struct for passing parameters to the method [`delete_rate_limiter`]
#[derive(Clone, Debug, Default)]
pub struct DeleteRateLimiterParams {
    /// Alphanumeric string identifying the rate limiter.
    pub rate_limiter_id: String
}

/// struct for passing parameters to the method [`get_rate_limiter`]
#[derive(Clone, Debug, Default)]
pub struct GetRateLimiterParams {
    /// Alphanumeric string identifying the rate limiter.
    pub rate_limiter_id: String
}

/// struct for passing parameters to the method [`list_rate_limiters`]
#[derive(Clone, Debug, Default)]
pub struct ListRateLimitersParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32
}

/// struct for passing parameters to the method [`update_rate_limiter`]
#[derive(Clone, Debug, Default)]
pub struct UpdateRateLimiterParams {
    /// Alphanumeric string identifying the rate limiter.
    pub rate_limiter_id: String,
    /// A human readable name for the rate limiting rule.
    pub name: Option<String>,
    /// The name of an Edge Dictionary containing URIs as keys. If not defined or `null`, all origin URIs will be rate limited.
    pub uri_dictionary_name: Option<String>,
    /// Array of HTTP methods to apply rate limiting to.
    pub http_methods: Option<Vec<String>>,
    /// Upper limit of requests per second allowed by the rate limiter.
    pub rps_limit: Option<i32>,
    /// Number of seconds during which the RPS limit must be exceeded in order to trigger a violation.
    pub window_size: Option<i32>,
    /// Array of VCL variables used to generate a counter key to identify a client. Example variables include `req.http.Fastly-Client-IP`, `req.http.User-Agent`, or a custom header like `req.http.API-Key`.
    pub client_key: Option<Vec<String>>,
    /// Length of time in minutes that the rate limiter is in effect after the initial violation is detected.
    pub penalty_box_duration: Option<i32>,
    /// The action to take when a rate limiter violation is detected.
    pub action: Option<String>,
    /// Name of existing response object. Required if `action` is `response_object`. Note that the rate limiter response is only updated to reflect the response object content when saving the rate limiter configuration.
    pub response_object_name: Option<String>,
    /// Name of the type of logging endpoint to be used when action is `log_only`. The logging endpoint type is used to determine the appropriate log format to use when emitting log entries.
    pub logger_type: Option<String>,
    /// Revision number of the rate limiting feature implementation. Defaults to the most recent revision.
    pub feature_revision: Option<i32>
}


/// struct for typed errors of method [`create_rate_limiter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRateLimiterError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_rate_limiter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRateLimiterError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_rate_limiter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRateLimiterError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_rate_limiters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRateLimitersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_rate_limiter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRateLimiterError {
    UnknownValue(serde_json::Value),
}


/// Create a rate limiter for a particular service and version.
pub async fn create_rate_limiter(configuration: &mut configuration::Configuration, params: CreateRateLimiterParams) -> Result<crate::models::RateLimiterResponse, Error<CreateRateLimiterError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let name = params.name;
    let uri_dictionary_name = params.uri_dictionary_name;
    let http_methods = params.http_methods;
    let rps_limit = params.rps_limit;
    let window_size = params.window_size;
    let client_key = params.client_key;
    let penalty_box_duration = params.penalty_box_duration;
    let action = params.action;
    let response_object_name = params.response_object_name;
    let logger_type = params.logger_type;
    let feature_revision = params.feature_revision;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/rate-limiters", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
    if let Some(local_var_param_value) = uri_dictionary_name {
        local_var_form_params.insert("uri_dictionary_name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = http_methods {
        local_var_form_params.insert("http_methods", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    if let Some(local_var_param_value) = rps_limit {
        local_var_form_params.insert("rps_limit", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = window_size {
        local_var_form_params.insert("window_size", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = client_key {
        local_var_form_params.insert("client_key", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    if let Some(local_var_param_value) = penalty_box_duration {
        local_var_form_params.insert("penalty_box_duration", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = action {
        local_var_form_params.insert("action", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = response_object_name {
        local_var_form_params.insert("response_object_name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = logger_type {
        local_var_form_params.insert("logger_type", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = feature_revision {
        local_var_form_params.insert("feature_revision", local_var_param_value.to_string());
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
        let local_var_entity: Option<CreateRateLimiterError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a rate limiter by its ID.
pub async fn delete_rate_limiter(configuration: &mut configuration::Configuration, params: DeleteRateLimiterParams) -> Result<crate::models::InlineResponse200, Error<DeleteRateLimiterError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let rate_limiter_id = params.rate_limiter_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rate-limiters/{rate_limiter_id}", local_var_configuration.base_path, rate_limiter_id=crate::apis::urlencode(rate_limiter_id));
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
        let local_var_entity: Option<DeleteRateLimiterError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a rate limiter by its ID.
pub async fn get_rate_limiter(configuration: &mut configuration::Configuration, params: GetRateLimiterParams) -> Result<crate::models::RateLimiterResponse, Error<GetRateLimiterError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let rate_limiter_id = params.rate_limiter_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rate-limiters/{rate_limiter_id}", local_var_configuration.base_path, rate_limiter_id=crate::apis::urlencode(rate_limiter_id));
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
        let local_var_entity: Option<GetRateLimiterError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all rate limiters for a particular service and version.
pub async fn list_rate_limiters(configuration: &mut configuration::Configuration, params: ListRateLimitersParams) -> Result<Vec<crate::models::RateLimiterResponse>, Error<ListRateLimitersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/rate-limiters", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
        let local_var_entity: Option<ListRateLimitersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a rate limiter by its ID.
pub async fn update_rate_limiter(configuration: &mut configuration::Configuration, params: UpdateRateLimiterParams) -> Result<crate::models::RateLimiterResponse, Error<UpdateRateLimiterError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let rate_limiter_id = params.rate_limiter_id;
    let name = params.name;
    let uri_dictionary_name = params.uri_dictionary_name;
    let http_methods = params.http_methods;
    let rps_limit = params.rps_limit;
    let window_size = params.window_size;
    let client_key = params.client_key;
    let penalty_box_duration = params.penalty_box_duration;
    let action = params.action;
    let response_object_name = params.response_object_name;
    let logger_type = params.logger_type;
    let feature_revision = params.feature_revision;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rate-limiters/{rate_limiter_id}", local_var_configuration.base_path, rate_limiter_id=crate::apis::urlencode(rate_limiter_id));
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
    if let Some(local_var_param_value) = uri_dictionary_name {
        local_var_form_params.insert("uri_dictionary_name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = http_methods {
        local_var_form_params.insert("http_methods", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    if let Some(local_var_param_value) = rps_limit {
        local_var_form_params.insert("rps_limit", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = window_size {
        local_var_form_params.insert("window_size", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = client_key {
        local_var_form_params.insert("client_key", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    if let Some(local_var_param_value) = penalty_box_duration {
        local_var_form_params.insert("penalty_box_duration", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = action {
        local_var_form_params.insert("action", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = response_object_name {
        local_var_form_params.insert("response_object_name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = logger_type {
        local_var_form_params.insert("logger_type", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = feature_revision {
        local_var_form_params.insert("feature_revision", local_var_param_value.to_string());
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
        let local_var_entity: Option<UpdateRateLimiterError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

