/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_pool_server`]
#[derive(Clone, Debug, Default)]
pub struct CreatePoolServerParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Pool.
    pub pool_id: String,
    /// Weight (`1-100`) used to load balance this server against others.
    pub weight: Option<i32>,
    /// Maximum number of connections. If the value is `0`, it inherits the value from pool's `max_conn_default`.
    pub max_conn: Option<i32>,
    /// Port number. Setting port `443` does not force TLS. Set `use_tls` in pool to force TLS.
    pub port: Option<i32>,
    /// A hostname, IPv4, or IPv6 address for the server. Required.
    pub address: Option<String>,
    /// A freeform descriptive note.
    pub comment: Option<String>,
    /// Allows servers to be enabled and disabled in a pool.
    pub disabled: Option<bool>,
    /// The hostname to override the Host header. Defaults to `null` meaning no override of the Host header if not set. This setting can also be added to a Pool definition. However, the server setting will override the Pool setting.
    pub override_host: Option<String>
}

/// struct for passing parameters to the method [`delete_pool_server`]
#[derive(Clone, Debug, Default)]
pub struct DeletePoolServerParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Pool.
    pub pool_id: String,
    /// Alphanumeric string identifying a Server.
    pub server_id: String
}

/// struct for passing parameters to the method [`get_pool_server`]
#[derive(Clone, Debug, Default)]
pub struct GetPoolServerParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Pool.
    pub pool_id: String,
    /// Alphanumeric string identifying a Server.
    pub server_id: String
}

/// struct for passing parameters to the method [`list_pool_servers`]
#[derive(Clone, Debug, Default)]
pub struct ListPoolServersParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Pool.
    pub pool_id: String
}

/// struct for passing parameters to the method [`update_pool_server`]
#[derive(Clone, Debug, Default)]
pub struct UpdatePoolServerParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Pool.
    pub pool_id: String,
    /// Alphanumeric string identifying a Server.
    pub server_id: String,
    /// Weight (`1-100`) used to load balance this server against others.
    pub weight: Option<i32>,
    /// Maximum number of connections. If the value is `0`, it inherits the value from pool's `max_conn_default`.
    pub max_conn: Option<i32>,
    /// Port number. Setting port `443` does not force TLS. Set `use_tls` in pool to force TLS.
    pub port: Option<i32>,
    /// A hostname, IPv4, or IPv6 address for the server. Required.
    pub address: Option<String>,
    /// A freeform descriptive note.
    pub comment: Option<String>,
    /// Allows servers to be enabled and disabled in a pool.
    pub disabled: Option<bool>,
    /// The hostname to override the Host header. Defaults to `null` meaning no override of the Host header if not set. This setting can also be added to a Pool definition. However, the server setting will override the Pool setting.
    pub override_host: Option<String>
}


/// struct for typed errors of method [`create_pool_server`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePoolServerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_pool_server`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePoolServerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_pool_server`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPoolServerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_pool_servers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPoolServersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_pool_server`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePoolServerError {
    UnknownValue(serde_json::Value),
}


/// Creates a single server for a particular service and pool.
pub async fn create_pool_server(configuration: &mut configuration::Configuration, params: CreatePoolServerParams) -> Result<crate::models::ServerResponse, Error<CreatePoolServerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let pool_id = params.pool_id;
    let weight = params.weight;
    let max_conn = params.max_conn;
    let port = params.port;
    let address = params.address;
    let comment = params.comment;
    let disabled = params.disabled;
    let override_host = params.override_host;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/pool/{pool_id}/server", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), pool_id=crate::apis::urlencode(pool_id));
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
    if let Some(local_var_param_value) = weight {
        local_var_form_params.insert("weight", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = max_conn {
        local_var_form_params.insert("max_conn", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = port {
        local_var_form_params.insert("port", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = address {
        local_var_form_params.insert("address", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comment {
        local_var_form_params.insert("comment", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = disabled {
        local_var_form_params.insert("disabled", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = override_host {
        local_var_form_params.insert("override_host", local_var_param_value.to_string());
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
        let local_var_entity: Option<CreatePoolServerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a single server for a particular service and pool.
pub async fn delete_pool_server(configuration: &mut configuration::Configuration, params: DeletePoolServerParams) -> Result<crate::models::InlineResponse200, Error<DeletePoolServerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let pool_id = params.pool_id;
    let server_id = params.server_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/pool/{pool_id}/server/{server_id}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), pool_id=crate::apis::urlencode(pool_id), server_id=crate::apis::urlencode(server_id));
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
        let local_var_entity: Option<DeletePoolServerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a single server for a particular service and pool.
pub async fn get_pool_server(configuration: &mut configuration::Configuration, params: GetPoolServerParams) -> Result<crate::models::ServerResponse, Error<GetPoolServerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let pool_id = params.pool_id;
    let server_id = params.server_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/pool/{pool_id}/server/{server_id}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), pool_id=crate::apis::urlencode(pool_id), server_id=crate::apis::urlencode(server_id));
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
        let local_var_entity: Option<GetPoolServerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all servers for a particular service and pool.
pub async fn list_pool_servers(configuration: &mut configuration::Configuration, params: ListPoolServersParams) -> Result<Vec<crate::models::ServerResponse>, Error<ListPoolServersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let pool_id = params.pool_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/pool/{pool_id}/servers", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), pool_id=crate::apis::urlencode(pool_id));
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
        let local_var_entity: Option<ListPoolServersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a single server for a particular service and pool.
pub async fn update_pool_server(configuration: &mut configuration::Configuration, params: UpdatePoolServerParams) -> Result<crate::models::ServerResponse, Error<UpdatePoolServerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let pool_id = params.pool_id;
    let server_id = params.server_id;
    let weight = params.weight;
    let max_conn = params.max_conn;
    let port = params.port;
    let address = params.address;
    let comment = params.comment;
    let disabled = params.disabled;
    let override_host = params.override_host;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/pool/{pool_id}/server/{server_id}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), pool_id=crate::apis::urlencode(pool_id), server_id=crate::apis::urlencode(server_id));
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
    if let Some(local_var_param_value) = weight {
        local_var_form_params.insert("weight", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = max_conn {
        local_var_form_params.insert("max_conn", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = port {
        local_var_form_params.insert("port", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = address {
        local_var_form_params.insert("address", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comment {
        local_var_form_params.insert("comment", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = disabled {
        local_var_form_params.insert("disabled", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = override_host {
        local_var_form_params.insert("override_host", local_var_param_value.to_string());
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
        let local_var_entity: Option<UpdatePoolServerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

