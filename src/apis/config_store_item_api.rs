/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`bulk_update_config_store_item`]
#[derive(Clone, Debug, Default)]
pub struct BulkUpdateConfigStoreItemParams {
    /// An alphanumeric string identifying the config store.
    pub config_store_id: String,
    pub bulk_update_config_store_list_request: Option<crate::models::BulkUpdateConfigStoreListRequest>
}

/// struct for passing parameters to the method [`create_config_store_item`]
#[derive(Clone, Debug, Default)]
pub struct CreateConfigStoreItemParams {
    /// An alphanumeric string identifying the config store.
    pub config_store_id: String,
    /// Item key, maximum 256 characters.
    pub item_key: Option<String>,
    /// Item value, maximum 8000 characters.
    pub item_value: Option<String>
}

/// struct for passing parameters to the method [`delete_config_store_item`]
#[derive(Clone, Debug, Default)]
pub struct DeleteConfigStoreItemParams {
    /// An alphanumeric string identifying the config store.
    pub config_store_id: String,
    /// Item key, maximum 256 characters.
    pub config_store_item_key: String
}

/// struct for passing parameters to the method [`get_config_store_item`]
#[derive(Clone, Debug, Default)]
pub struct GetConfigStoreItemParams {
    /// An alphanumeric string identifying the config store.
    pub config_store_id: String,
    /// Item key, maximum 256 characters.
    pub config_store_item_key: String
}

/// struct for passing parameters to the method [`list_config_store_items`]
#[derive(Clone, Debug, Default)]
pub struct ListConfigStoreItemsParams {
    /// An alphanumeric string identifying the config store.
    pub config_store_id: String
}

/// struct for passing parameters to the method [`update_config_store_item`]
#[derive(Clone, Debug, Default)]
pub struct UpdateConfigStoreItemParams {
    /// An alphanumeric string identifying the config store.
    pub config_store_id: String,
    /// Item key, maximum 256 characters.
    pub config_store_item_key: String,
    /// Item key, maximum 256 characters.
    pub item_key: Option<String>,
    /// Item value, maximum 8000 characters.
    pub item_value: Option<String>
}

/// struct for passing parameters to the method [`upsert_config_store_item`]
#[derive(Clone, Debug, Default)]
pub struct UpsertConfigStoreItemParams {
    /// An alphanumeric string identifying the config store.
    pub config_store_id: String,
    /// Item key, maximum 256 characters.
    pub config_store_item_key: String,
    /// Item key, maximum 256 characters.
    pub item_key: Option<String>,
    /// Item value, maximum 8000 characters.
    pub item_value: Option<String>
}


/// struct for typed errors of method [`bulk_update_config_store_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateConfigStoreItemError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_config_store_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateConfigStoreItemError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_config_store_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteConfigStoreItemError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_config_store_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetConfigStoreItemError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_config_store_items`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListConfigStoreItemsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_config_store_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateConfigStoreItemError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upsert_config_store_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpsertConfigStoreItemError {
    UnknownValue(serde_json::Value),
}


/// Add multiple key-value pairs to an individual config store, specified by ID.
pub async fn bulk_update_config_store_item(configuration: &mut configuration::Configuration, params: BulkUpdateConfigStoreItemParams) -> Result<crate::models::InlineResponse200, Error<BulkUpdateConfigStoreItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_store_id = params.config_store_id;
    let bulk_update_config_store_list_request = params.bulk_update_config_store_list_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/resources/stores/config/{config_store_id}/items", local_var_configuration.base_path, config_store_id=crate::apis::urlencode(config_store_id));
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
    local_var_req_builder = local_var_req_builder.json(&bulk_update_config_store_list_request);

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
        let local_var_entity: Option<BulkUpdateConfigStoreItemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Add a single key-value pair to an individual config store, specified by ID.
pub async fn create_config_store_item(configuration: &mut configuration::Configuration, params: CreateConfigStoreItemParams) -> Result<crate::models::ConfigStoreItemResponse, Error<CreateConfigStoreItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_store_id = params.config_store_id;
    let item_key = params.item_key;
    let item_value = params.item_value;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/resources/stores/config/{config_store_id}/item", local_var_configuration.base_path, config_store_id=crate::apis::urlencode(config_store_id));
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
    if let Some(local_var_param_value) = item_key {
        local_var_form_params.insert("item_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = item_value {
        local_var_form_params.insert("item_value", local_var_param_value.to_string());
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
        let local_var_entity: Option<CreateConfigStoreItemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an entry in a config store given a config store ID, and item key.
pub async fn delete_config_store_item(configuration: &mut configuration::Configuration, params: DeleteConfigStoreItemParams) -> Result<crate::models::InlineResponse200, Error<DeleteConfigStoreItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_store_id = params.config_store_id;
    let config_store_item_key = params.config_store_item_key;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/resources/stores/config/{config_store_id}/item/{config_store_item_key}", local_var_configuration.base_path, config_store_id=crate::apis::urlencode(config_store_id), config_store_item_key=crate::apis::urlencode(config_store_item_key));
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
        let local_var_entity: Option<DeleteConfigStoreItemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a config store entry given a config store ID and item key.
pub async fn get_config_store_item(configuration: &mut configuration::Configuration, params: GetConfigStoreItemParams) -> Result<crate::models::ConfigStoreItemResponse, Error<GetConfigStoreItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_store_id = params.config_store_id;
    let config_store_item_key = params.config_store_item_key;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/resources/stores/config/{config_store_id}/item/{config_store_item_key}", local_var_configuration.base_path, config_store_id=crate::apis::urlencode(config_store_id), config_store_item_key=crate::apis::urlencode(config_store_item_key));
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
        let local_var_entity: Option<GetConfigStoreItemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List the key-value pairs associated with a given config store ID.
pub async fn list_config_store_items(configuration: &mut configuration::Configuration, params: ListConfigStoreItemsParams) -> Result<Vec<crate::models::ConfigStoreItemResponse>, Error<ListConfigStoreItemsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_store_id = params.config_store_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/resources/stores/config/{config_store_id}/items", local_var_configuration.base_path, config_store_id=crate::apis::urlencode(config_store_id));
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
        let local_var_entity: Option<ListConfigStoreItemsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update an entry in a config store given a config store ID, item key, and item value.
pub async fn update_config_store_item(configuration: &mut configuration::Configuration, params: UpdateConfigStoreItemParams) -> Result<crate::models::ConfigStoreItemResponse, Error<UpdateConfigStoreItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_store_id = params.config_store_id;
    let config_store_item_key = params.config_store_item_key;
    let item_key = params.item_key;
    let item_value = params.item_value;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/resources/stores/config/{config_store_id}/item/{config_store_item_key}", local_var_configuration.base_path, config_store_id=crate::apis::urlencode(config_store_id), config_store_item_key=crate::apis::urlencode(config_store_item_key));
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
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = item_key {
        local_var_form_params.insert("item_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = item_value {
        local_var_form_params.insert("item_value", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

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
        let local_var_entity: Option<UpdateConfigStoreItemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Insert or update an entry in a config store given a config store ID, item key, and item value.
pub async fn upsert_config_store_item(configuration: &mut configuration::Configuration, params: UpsertConfigStoreItemParams) -> Result<crate::models::ConfigStoreItemResponse, Error<UpsertConfigStoreItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_store_id = params.config_store_id;
    let config_store_item_key = params.config_store_item_key;
    let item_key = params.item_key;
    let item_value = params.item_value;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/resources/stores/config/{config_store_id}/item/{config_store_item_key}", local_var_configuration.base_path, config_store_id=crate::apis::urlencode(config_store_id), config_store_item_key=crate::apis::urlencode(config_store_item_key));
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
    if let Some(local_var_param_value) = item_key {
        local_var_form_params.insert("item_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = item_value {
        local_var_form_params.insert("item_value", local_var_param_value.to_string());
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
        let local_var_entity: Option<UpsertConfigStoreItemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

