/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_request_settings`]
#[derive(Clone, Debug, Default)]
pub struct CreateRequestSettingsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32
}

/// struct for passing parameters to the method [`delete_request_settings`]
#[derive(Clone, Debug, Default)]
pub struct DeleteRequestSettingsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// Name for the request settings.
    pub request_settings_name: String
}

/// struct for passing parameters to the method [`get_request_settings`]
#[derive(Clone, Debug, Default)]
pub struct GetRequestSettingsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// Name for the request settings.
    pub request_settings_name: String
}

/// struct for passing parameters to the method [`list_request_settings`]
#[derive(Clone, Debug, Default)]
pub struct ListRequestSettingsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32
}

/// struct for passing parameters to the method [`update_request_settings`]
#[derive(Clone, Debug, Default)]
pub struct UpdateRequestSettingsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// Name for the request settings.
    pub request_settings_name: String,
    /// Allows you to terminate request handling and immediately perform an action.
    pub action: Option<String>,
    /// Disable collapsed forwarding, so you don't wait for other objects to origin.
    pub bypass_busy_wait: Option<i32>,
    /// Sets the host header.
    pub default_host: Option<String>,
    /// Allows you to force a cache miss for the request. Replaces the item in the cache if the content is cacheable.
    pub force_miss: Option<i32>,
    /// Forces the request use SSL (redirects a non-SSL to SSL).
    pub force_ssl: Option<i32>,
    /// Injects Fastly-Geo-Country, Fastly-Geo-City, and Fastly-Geo-Region into the request headers.
    pub geo_headers: Option<i32>,
    /// Comma separated list of varnish request object fields that should be in the hash key.
    pub hash_keys: Option<String>,
    /// How old an object is allowed to be to serve stale-if-error or stale-while-revalidate.
    pub max_stale_age: Option<i32>,
    /// Name for the request settings.
    pub name: Option<String>,
    /// Condition which, if met, will select this configuration during a request. Optional.
    pub request_condition: Option<String>,
    /// Injects the X-Timer info into the request for viewing origin fetch durations.
    pub timer_support: Option<i32>,
    /// Short for X-Forwarded-For.
    pub xff: Option<String>
}


/// struct for typed errors of method [`create_request_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRequestSettingsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_request_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRequestSettingsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_request_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRequestSettingsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_request_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRequestSettingsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_request_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRequestSettingsError {
    UnknownValue(serde_json::Value),
}


/// Creates a new Request Settings object.
pub async fn create_request_settings(configuration: &mut configuration::Configuration, params: CreateRequestSettingsParams) -> Result<crate::models::RequestSettingsResponse, Error<CreateRequestSettingsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/request_settings", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
        let local_var_entity: Option<CreateRequestSettingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Removes the specified Request Settings object.
pub async fn delete_request_settings(configuration: &mut configuration::Configuration, params: DeleteRequestSettingsParams) -> Result<crate::models::InlineResponse200, Error<DeleteRequestSettingsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let request_settings_name = params.request_settings_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/request_settings/{request_settings_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, request_settings_name=crate::apis::urlencode(request_settings_name));
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
        let local_var_entity: Option<DeleteRequestSettingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets the specified Request Settings object.
pub async fn get_request_settings(configuration: &mut configuration::Configuration, params: GetRequestSettingsParams) -> Result<crate::models::RequestSettingsResponse, Error<GetRequestSettingsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let request_settings_name = params.request_settings_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/request_settings/{request_settings_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, request_settings_name=crate::apis::urlencode(request_settings_name));
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
        let local_var_entity: Option<GetRequestSettingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of all Request Settings objects for the given service and version.
pub async fn list_request_settings(configuration: &mut configuration::Configuration, params: ListRequestSettingsParams) -> Result<Vec<crate::models::RequestSettingsResponse>, Error<ListRequestSettingsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/request_settings", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
        let local_var_entity: Option<ListRequestSettingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the specified Request Settings object.
pub async fn update_request_settings(configuration: &mut configuration::Configuration, params: UpdateRequestSettingsParams) -> Result<crate::models::RequestSettingsResponse, Error<UpdateRequestSettingsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let request_settings_name = params.request_settings_name;
    let action = params.action;
    let bypass_busy_wait = params.bypass_busy_wait;
    let default_host = params.default_host;
    let force_miss = params.force_miss;
    let force_ssl = params.force_ssl;
    let geo_headers = params.geo_headers;
    let hash_keys = params.hash_keys;
    let max_stale_age = params.max_stale_age;
    let name = params.name;
    let request_condition = params.request_condition;
    let timer_support = params.timer_support;
    let xff = params.xff;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/request_settings/{request_settings_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, request_settings_name=crate::apis::urlencode(request_settings_name));
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
    if let Some(local_var_param_value) = action {
        local_var_form_params.insert("action", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = bypass_busy_wait {
        local_var_form_params.insert("bypass_busy_wait", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = default_host {
        local_var_form_params.insert("default_host", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = force_miss {
        local_var_form_params.insert("force_miss", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = force_ssl {
        local_var_form_params.insert("force_ssl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = geo_headers {
        local_var_form_params.insert("geo_headers", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = hash_keys {
        local_var_form_params.insert("hash_keys", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = max_stale_age {
        local_var_form_params.insert("max_stale_age", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = request_condition {
        local_var_form_params.insert("request_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = timer_support {
        local_var_form_params.insert("timer_support", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = xff {
        local_var_form_params.insert("xff", local_var_param_value.to_string());
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
        let local_var_entity: Option<UpdateRequestSettingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

