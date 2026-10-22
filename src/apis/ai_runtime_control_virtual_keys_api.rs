/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`arc_create_virtual_key`]
#[derive(Clone, Debug, Default)]
pub struct ArcCreateVirtualKeyParams {
    pub arc_virtual_key: crate::models::ArcVirtualKey
}

/// struct for passing parameters to the method [`arc_delete_virtual_key`]
#[derive(Clone, Debug, Default)]
pub struct ArcDeleteVirtualKeyParams {
    /// The ID of the virtual key.
    pub key_id: String
}

/// struct for passing parameters to the method [`arc_get_virtual_key`]
#[derive(Clone, Debug, Default)]
pub struct ArcGetVirtualKeyParams {
    /// The ID of the virtual key.
    pub key_id: String
}

/// struct for passing parameters to the method [`arc_get_virtual_key_failover`]
#[derive(Clone, Debug, Default)]
pub struct ArcGetVirtualKeyFailoverParams {
    /// The ID of the virtual key.
    pub key_id: String
}

/// struct for passing parameters to the method [`arc_list_virtual_keys`]
#[derive(Clone, Debug, Default)]
pub struct ArcListVirtualKeysParams {
    /// Include deleted virtual keys in the results.
    pub include_deleted: Option<bool>,
    /// Filter results to virtual keys whose name contains this substring.
    pub search: Option<String>,
    /// Cursor for pagination. Use the `next_cursor` value from a previous response to request the next page of results, or the `prev_cursor` value to request the previous page. 
    pub cursor: Option<String>,
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Sort field. Prefix with `-` for descending order (e.g., `sort=-created_at`). 
    pub sort: Option<String>
}

/// struct for passing parameters to the method [`arc_rotate_virtual_key`]
#[derive(Clone, Debug, Default)]
pub struct ArcRotateVirtualKeyParams {
    /// The ID of the virtual key.
    pub key_id: String,
    pub arc_rotate_virtual_key_request: Option<crate::models::ArcRotateVirtualKeyRequest>
}

/// struct for passing parameters to the method [`arc_update_virtual_key`]
#[derive(Clone, Debug, Default)]
pub struct ArcUpdateVirtualKeyParams {
    /// The ID of the virtual key.
    pub key_id: String,
    pub arc_update_virtual_key_request: crate::models::ArcUpdateVirtualKeyRequest
}

/// struct for passing parameters to the method [`arc_update_virtual_key_failover`]
#[derive(Clone, Debug, Default)]
pub struct ArcUpdateVirtualKeyFailoverParams {
    /// The ID of the virtual key.
    pub key_id: String,
    pub arc_virtual_key_failover_request: crate::models::ArcVirtualKeyFailoverRequest
}


/// struct for typed errors of method [`arc_create_virtual_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArcCreateVirtualKeyError {
    Status400(crate::models::ArcError),
    Status401(crate::models::ArcError),
    Status403(crate::models::ArcError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`arc_delete_virtual_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArcDeleteVirtualKeyError {
    Status401(crate::models::ArcError),
    Status403(crate::models::ArcError),
    Status404(crate::models::ArcError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`arc_get_virtual_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArcGetVirtualKeyError {
    Status401(crate::models::ArcError),
    Status403(crate::models::ArcError),
    Status404(crate::models::ArcError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`arc_get_virtual_key_failover`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArcGetVirtualKeyFailoverError {
    Status401(crate::models::ArcError),
    Status403(crate::models::ArcError),
    Status404(crate::models::ArcError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`arc_list_virtual_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArcListVirtualKeysError {
    Status401(crate::models::ArcError),
    Status403(crate::models::ArcError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`arc_rotate_virtual_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArcRotateVirtualKeyError {
    Status400(crate::models::ArcError),
    Status401(crate::models::ArcError),
    Status403(crate::models::ArcError),
    Status404(crate::models::ArcError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`arc_update_virtual_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArcUpdateVirtualKeyError {
    Status400(crate::models::ArcError),
    Status401(crate::models::ArcError),
    Status403(crate::models::ArcError),
    Status404(crate::models::ArcError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`arc_update_virtual_key_failover`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArcUpdateVirtualKeyFailoverError {
    Status400(crate::models::ArcError),
    Status401(crate::models::ArcError),
    Status403(crate::models::ArcError),
    Status404(crate::models::ArcError),
    UnknownValue(serde_json::Value),
}


/// Create a new virtual key for authenticating requests through the AI Runtime Control gateway.
pub async fn arc_create_virtual_key(configuration: &mut configuration::Configuration, params: ArcCreateVirtualKeyParams) -> Result<crate::models::ArcCreateVirtualKeyResponse, Error<ArcCreateVirtualKeyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let arc_virtual_key = params.arc_virtual_key;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ai-runtime-control/v1/keys", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&arc_virtual_key);

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
        let local_var_entity: Option<ArcCreateVirtualKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a virtual key. This also deletes any security configuration associated with the key.
pub async fn arc_delete_virtual_key(configuration: &mut configuration::Configuration, params: ArcDeleteVirtualKeyParams) -> Result<(), Error<ArcDeleteVirtualKeyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let key_id = params.key_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ai-runtime-control/v1/keys/{key_id}", local_var_configuration.base_path, key_id=crate::apis::urlencode(key_id));
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
        Ok(())
    } else {
        let local_var_entity: Option<ArcDeleteVirtualKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a single virtual key.
pub async fn arc_get_virtual_key(configuration: &mut configuration::Configuration, params: ArcGetVirtualKeyParams) -> Result<crate::models::ArcVirtualKeyResponse, Error<ArcGetVirtualKeyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let key_id = params.key_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ai-runtime-control/v1/keys/{key_id}", local_var_configuration.base_path, key_id=crate::apis::urlencode(key_id));
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
        let local_var_entity: Option<ArcGetVirtualKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the failover policy for a virtual key. A failover policy lists alternate provider connections to retry a request against, and which error conditions trigger a retry.  Failover is a per-account feature. If it isn't enabled on your account, this endpoint returns a 404 response. 
pub async fn arc_get_virtual_key_failover(configuration: &mut configuration::Configuration, params: ArcGetVirtualKeyFailoverParams) -> Result<crate::models::ArcVirtualKeyFailoverResponse, Error<ArcGetVirtualKeyFailoverError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let key_id = params.key_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ai-runtime-control/v1/keys/{key_id}/failover", local_var_configuration.base_path, key_id=crate::apis::urlencode(key_id));
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
        let local_var_entity: Option<ArcGetVirtualKeyFailoverError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List the virtual keys belonging to your account.
pub async fn arc_list_virtual_keys(configuration: &mut configuration::Configuration, params: ArcListVirtualKeysParams) -> Result<crate::models::ArcVirtualKeysResponse, Error<ArcListVirtualKeysError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let include_deleted = params.include_deleted;
    let search = params.search;
    let cursor = params.cursor;
    let limit = params.limit;
    let sort = params.sort;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ai-runtime-control/v1/keys", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include_deleted {
        local_var_req_builder = local_var_req_builder.query(&[("include_deleted", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder = local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<ArcListVirtualKeysError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Issue a new access token for a virtual key, invalidating the previous one.
pub async fn arc_rotate_virtual_key(configuration: &mut configuration::Configuration, params: ArcRotateVirtualKeyParams) -> Result<crate::models::ArcRotateVirtualKeyResponse, Error<ArcRotateVirtualKeyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let key_id = params.key_id;
    let arc_rotate_virtual_key_request = params.arc_rotate_virtual_key_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ai-runtime-control/v1/keys/{key_id}/rotate", local_var_configuration.base_path, key_id=crate::apis::urlencode(key_id));
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
    local_var_req_builder = local_var_req_builder.json(&arc_rotate_virtual_key_request);

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
        let local_var_entity: Option<ArcRotateVirtualKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the attributes of a virtual key. Only the fields you include are changed.
pub async fn arc_update_virtual_key(configuration: &mut configuration::Configuration, params: ArcUpdateVirtualKeyParams) -> Result<crate::models::ArcUpdateVirtualKeyResponse, Error<ArcUpdateVirtualKeyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let key_id = params.key_id;
    let arc_update_virtual_key_request = params.arc_update_virtual_key_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ai-runtime-control/v1/keys/{key_id}", local_var_configuration.base_path, key_id=crate::apis::urlencode(key_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&arc_update_virtual_key_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "PATCH" != "GET" && "PATCH" != "HEAD" {
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
        let local_var_entity: Option<ArcUpdateVirtualKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Replace the failover policy for a virtual key.  Failover is a per-account feature. If it isn't enabled on your account, this endpoint returns a 404 response. 
pub async fn arc_update_virtual_key_failover(configuration: &mut configuration::Configuration, params: ArcUpdateVirtualKeyFailoverParams) -> Result<crate::models::ArcVirtualKeyFailoverResponse, Error<ArcUpdateVirtualKeyFailoverError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let key_id = params.key_id;
    let arc_virtual_key_failover_request = params.arc_virtual_key_failover_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ai-runtime-control/v1/keys/{key_id}/failover", local_var_configuration.base_path, key_id=crate::apis::urlencode(key_id));
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
    local_var_req_builder = local_var_req_builder.json(&arc_virtual_key_failover_request);

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
        let local_var_entity: Option<ArcUpdateVirtualKeyFailoverError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

