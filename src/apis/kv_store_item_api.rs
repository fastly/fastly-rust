/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`delete_key_from_store`]
#[derive(Clone, Debug, Default)]
pub struct DeleteKeyFromStoreParams {
    pub store_id: String,
    pub key_name: String
}

/// struct for passing parameters to the method [`get_keys`]
#[derive(Clone, Debug, Default)]
pub struct GetKeysParams {
    pub store_id: String,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub prefix: Option<String>
}

/// struct for passing parameters to the method [`get_value_for_key`]
#[derive(Clone, Debug, Default)]
pub struct GetValueForKeyParams {
    pub store_id: String,
    pub key_name: String
}

/// struct for passing parameters to the method [`set_value_for_key`]
#[derive(Clone, Debug, Default)]
pub struct SetValueForKeyParams {
    pub store_id: String,
    pub key_name: String,
    pub if_generation_match: Option<i32>,
    pub time_to_live_sec: Option<i32>,
    pub metadata: Option<String>,
    pub add: Option<bool>,
    pub append: Option<bool>,
    pub prepend: Option<bool>,
    pub background_fetch: Option<bool>,
    pub body: Option<String>
}


/// struct for typed errors of method [`delete_key_from_store`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteKeyFromStoreError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetKeysError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_value_for_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetValueForKeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_value_for_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetValueForKeyError {
    UnknownValue(serde_json::Value),
}


/// Delete an item from an kv store
pub async fn delete_key_from_store(configuration: &mut configuration::Configuration, params: DeleteKeyFromStoreParams) -> Result<(), Error<DeleteKeyFromStoreError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let store_id = params.store_id;
    let key_name = params.key_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/resources/stores/kv/{store_id}/keys/{key_name}", local_var_configuration.base_path, store_id=crate::apis::urlencode(store_id), key_name=crate::apis::urlencode(key_name));
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
        let local_var_entity: Option<DeleteKeyFromStoreError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List the keys of all items within an kv store.
pub async fn get_keys(configuration: &mut configuration::Configuration, params: GetKeysParams) -> Result<crate::models::InlineResponse2004, Error<GetKeysError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let store_id = params.store_id;
    let cursor = params.cursor;
    let limit = params.limit;
    let prefix = params.prefix;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/resources/stores/kv/{store_id}/keys", local_var_configuration.base_path, store_id=crate::apis::urlencode(store_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = prefix {
        local_var_req_builder = local_var_req_builder.query(&[("prefix", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetKeysError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the value associated with a key.
pub async fn get_value_for_key(configuration: &mut configuration::Configuration, params: GetValueForKeyParams) -> Result<String, Error<GetValueForKeyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let store_id = params.store_id;
    let key_name = params.key_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/resources/stores/kv/{store_id}/keys/{key_name}", local_var_configuration.base_path, store_id=crate::apis::urlencode(store_id), key_name=crate::apis::urlencode(key_name));
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
        let local_var_entity: Option<GetValueForKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Set a new value for a new or existing key in an kv store.
pub async fn set_value_for_key(configuration: &mut configuration::Configuration, params: SetValueForKeyParams) -> Result<String, Error<SetValueForKeyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let store_id = params.store_id;
    let key_name = params.key_name;
    let if_generation_match = params.if_generation_match;
    let time_to_live_sec = params.time_to_live_sec;
    let metadata = params.metadata;
    let add = params.add;
    let append = params.append;
    let prepend = params.prepend;
    let background_fetch = params.background_fetch;
    let body = params.body;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/resources/stores/kv/{store_id}/keys/{key_name}", local_var_configuration.base_path, store_id=crate::apis::urlencode(store_id), key_name=crate::apis::urlencode(key_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = add {
        local_var_req_builder = local_var_req_builder.query(&[("add", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = append {
        local_var_req_builder = local_var_req_builder.query(&[("append", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = prepend {
        local_var_req_builder = local_var_req_builder.query(&[("prepend", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = background_fetch {
        local_var_req_builder = local_var_req_builder.query(&[("background_fetch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_generation_match {
        local_var_req_builder = local_var_req_builder.header("if-generation-match", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = time_to_live_sec {
        local_var_req_builder = local_var_req_builder.header("time_to_live_sec", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = metadata {
        local_var_req_builder = local_var_req_builder.header("metadata", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Fastly-Key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

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
        let local_var_entity: Option<SetValueForKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

