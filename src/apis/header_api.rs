/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_header_object`]
#[derive(Clone, Debug, Default)]
pub struct CreateHeaderObjectParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// Accepts a string value.
    pub action: Option<String>,
    /// Name of the cache condition controlling when this configuration applies.
    pub cache_condition: Option<String>,
    /// Header to set.
    pub dst: Option<String>,
    /// Don't add the header if it is added already. Only applies to 'set' action.
    pub ignore_if_set: Option<i32>,
    /// A handle to refer to this Header object.
    pub name: Option<String>,
    /// Priority determines execution order. Lower numbers execute first.
    pub priority: Option<i32>,
    /// Regular expression to use. Only applies to `regex` and `regex_repeat` actions.
    pub regex: Option<String>,
    /// Condition which, if met, will select this configuration during a request. Optional.
    pub request_condition: Option<String>,
    /// Optional name of a response condition to apply.
    pub response_condition: Option<String>,
    /// Variable to be used as a source for the header content. Does not apply to `delete` action.
    pub src: Option<String>,
    /// Value to substitute in place of regular expression. Only applies to `regex` and `regex_repeat` actions.
    pub substitution: Option<String>,
    /// Accepts a string value.
    pub _type: Option<String>
}

/// struct for passing parameters to the method [`delete_header_object`]
#[derive(Clone, Debug, Default)]
pub struct DeleteHeaderObjectParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// A handle to refer to this Header object.
    pub header_name: String
}

/// struct for passing parameters to the method [`get_header_object`]
#[derive(Clone, Debug, Default)]
pub struct GetHeaderObjectParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// A handle to refer to this Header object.
    pub header_name: String
}

/// struct for passing parameters to the method [`list_header_objects`]
#[derive(Clone, Debug, Default)]
pub struct ListHeaderObjectsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32
}

/// struct for passing parameters to the method [`update_header_object`]
#[derive(Clone, Debug, Default)]
pub struct UpdateHeaderObjectParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// A handle to refer to this Header object.
    pub header_name: String,
    /// Accepts a string value.
    pub action: Option<String>,
    /// Name of the cache condition controlling when this configuration applies.
    pub cache_condition: Option<String>,
    /// Header to set.
    pub dst: Option<String>,
    /// Don't add the header if it is added already. Only applies to 'set' action.
    pub ignore_if_set: Option<i32>,
    /// A handle to refer to this Header object.
    pub name: Option<String>,
    /// Priority determines execution order. Lower numbers execute first.
    pub priority: Option<i32>,
    /// Regular expression to use. Only applies to `regex` and `regex_repeat` actions.
    pub regex: Option<String>,
    /// Condition which, if met, will select this configuration during a request. Optional.
    pub request_condition: Option<String>,
    /// Optional name of a response condition to apply.
    pub response_condition: Option<String>,
    /// Variable to be used as a source for the header content. Does not apply to `delete` action.
    pub src: Option<String>,
    /// Value to substitute in place of regular expression. Only applies to `regex` and `regex_repeat` actions.
    pub substitution: Option<String>,
    /// Accepts a string value.
    pub _type: Option<String>
}


/// struct for typed errors of method [`create_header_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateHeaderObjectError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_header_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteHeaderObjectError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_header_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHeaderObjectError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_header_objects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHeaderObjectsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_header_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateHeaderObjectError {
    UnknownValue(serde_json::Value),
}


/// Creates a new Header object.
pub async fn create_header_object(configuration: &mut configuration::Configuration, params: CreateHeaderObjectParams) -> Result<crate::models::HeaderResponse, Error<CreateHeaderObjectError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let action = params.action;
    let cache_condition = params.cache_condition;
    let dst = params.dst;
    let ignore_if_set = params.ignore_if_set;
    let name = params.name;
    let priority = params.priority;
    let regex = params.regex;
    let request_condition = params.request_condition;
    let response_condition = params.response_condition;
    let src = params.src;
    let substitution = params.substitution;
    let _type = params._type;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/header", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
    if let Some(local_var_param_value) = action {
        local_var_form_params.insert("action", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = cache_condition {
        local_var_form_params.insert("cache_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = dst {
        local_var_form_params.insert("dst", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ignore_if_set {
        local_var_form_params.insert("ignore_if_set", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = priority {
        local_var_form_params.insert("priority", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = regex {
        local_var_form_params.insert("regex", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = request_condition {
        local_var_form_params.insert("request_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = response_condition {
        local_var_form_params.insert("response_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = src {
        local_var_form_params.insert("src", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = substitution {
        local_var_form_params.insert("substitution", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = _type {
        local_var_form_params.insert("type", local_var_param_value.to_string());
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
        let local_var_entity: Option<CreateHeaderObjectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a Header object by name.
pub async fn delete_header_object(configuration: &mut configuration::Configuration, params: DeleteHeaderObjectParams) -> Result<crate::models::InlineResponse200, Error<DeleteHeaderObjectError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let header_name = params.header_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/header/{header_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, header_name=crate::apis::urlencode(header_name));
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
        let local_var_entity: Option<DeleteHeaderObjectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a Header object by name.
pub async fn get_header_object(configuration: &mut configuration::Configuration, params: GetHeaderObjectParams) -> Result<crate::models::HeaderResponse, Error<GetHeaderObjectError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let header_name = params.header_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/header/{header_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, header_name=crate::apis::urlencode(header_name));
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
        let local_var_entity: Option<GetHeaderObjectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves all Header objects for a particular Version of a Service.
pub async fn list_header_objects(configuration: &mut configuration::Configuration, params: ListHeaderObjectsParams) -> Result<Vec<crate::models::HeaderResponse>, Error<ListHeaderObjectsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/header", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
        let local_var_entity: Option<ListHeaderObjectsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Modifies an existing Header object by name.
pub async fn update_header_object(configuration: &mut configuration::Configuration, params: UpdateHeaderObjectParams) -> Result<crate::models::HeaderResponse, Error<UpdateHeaderObjectError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let header_name = params.header_name;
    let action = params.action;
    let cache_condition = params.cache_condition;
    let dst = params.dst;
    let ignore_if_set = params.ignore_if_set;
    let name = params.name;
    let priority = params.priority;
    let regex = params.regex;
    let request_condition = params.request_condition;
    let response_condition = params.response_condition;
    let src = params.src;
    let substitution = params.substitution;
    let _type = params._type;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/header/{header_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, header_name=crate::apis::urlencode(header_name));
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
    if let Some(local_var_param_value) = cache_condition {
        local_var_form_params.insert("cache_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = dst {
        local_var_form_params.insert("dst", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ignore_if_set {
        local_var_form_params.insert("ignore_if_set", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = priority {
        local_var_form_params.insert("priority", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = regex {
        local_var_form_params.insert("regex", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = request_condition {
        local_var_form_params.insert("request_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = response_condition {
        local_var_form_params.insert("response_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = src {
        local_var_form_params.insert("src", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = substitution {
        local_var_form_params.insert("substitution", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = _type {
        local_var_form_params.insert("type", local_var_param_value.to_string());
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
        let local_var_entity: Option<UpdateHeaderObjectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

