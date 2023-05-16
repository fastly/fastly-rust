/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`bulk_update_dictionary_item`]
#[derive(Clone, Debug, Default)]
pub struct BulkUpdateDictionaryItemParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Dictionary.
    pub dictionary_id: String,
    pub bulk_update_dictionary_list_request: Option<crate::models::BulkUpdateDictionaryListRequest>
}

/// struct for passing parameters to the method [`create_dictionary_item`]
#[derive(Clone, Debug, Default)]
pub struct CreateDictionaryItemParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Dictionary.
    pub dictionary_id: String,
    /// Item key, maximum 256 characters.
    pub item_key: Option<String>,
    /// Item value, maximum 8000 characters.
    pub item_value: Option<String>
}

/// struct for passing parameters to the method [`delete_dictionary_item`]
#[derive(Clone, Debug, Default)]
pub struct DeleteDictionaryItemParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Dictionary.
    pub dictionary_id: String,
    /// Item key, maximum 256 characters.
    pub dictionary_item_key: String
}

/// struct for passing parameters to the method [`get_dictionary_item`]
#[derive(Clone, Debug, Default)]
pub struct GetDictionaryItemParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Dictionary.
    pub dictionary_id: String,
    /// Item key, maximum 256 characters.
    pub dictionary_item_key: String
}

/// struct for passing parameters to the method [`list_dictionary_items`]
#[derive(Clone, Debug, Default)]
pub struct ListDictionaryItemsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Dictionary.
    pub dictionary_id: String,
    /// Current page.
    pub page: Option<i32>,
    /// Number of records per page.
    pub per_page: Option<i32>,
    /// Field on which to sort.
    pub sort: Option<String>,
    /// Direction in which to sort results.
    pub direction: Option<String>
}

/// struct for passing parameters to the method [`update_dictionary_item`]
#[derive(Clone, Debug, Default)]
pub struct UpdateDictionaryItemParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Dictionary.
    pub dictionary_id: String,
    /// Item key, maximum 256 characters.
    pub dictionary_item_key: String,
    /// Item key, maximum 256 characters.
    pub item_key: Option<String>,
    /// Item value, maximum 8000 characters.
    pub item_value: Option<String>
}

/// struct for passing parameters to the method [`upsert_dictionary_item`]
#[derive(Clone, Debug, Default)]
pub struct UpsertDictionaryItemParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Dictionary.
    pub dictionary_id: String,
    /// Item key, maximum 256 characters.
    pub dictionary_item_key: String,
    /// Item key, maximum 256 characters.
    pub item_key: Option<String>,
    /// Item value, maximum 8000 characters.
    pub item_value: Option<String>
}


/// struct for typed errors of method [`bulk_update_dictionary_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateDictionaryItemError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_dictionary_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDictionaryItemError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_dictionary_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDictionaryItemError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dictionary_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDictionaryItemError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_dictionary_items`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDictionaryItemsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_dictionary_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDictionaryItemError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upsert_dictionary_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpsertDictionaryItemError {
    UnknownValue(serde_json::Value),
}


/// Update multiple items in the same dictionary. For faster updates to your service, group your changes into large batches. The maximum batch size is 1000 items. [Contact support](https://support.fastly.com/) to discuss raising this limit.
pub async fn bulk_update_dictionary_item(configuration: &mut configuration::Configuration, params: BulkUpdateDictionaryItemParams) -> Result<crate::models::InlineResponse200, Error<BulkUpdateDictionaryItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let dictionary_id = params.dictionary_id;
    let bulk_update_dictionary_list_request = params.bulk_update_dictionary_list_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/dictionary/{dictionary_id}/items", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), dictionary_id=crate::apis::urlencode(dictionary_id));
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
    local_var_req_builder = local_var_req_builder.json(&bulk_update_dictionary_list_request);

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
        let local_var_entity: Option<BulkUpdateDictionaryItemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create DictionaryItem given service, dictionary ID, item key, and item value.
pub async fn create_dictionary_item(configuration: &mut configuration::Configuration, params: CreateDictionaryItemParams) -> Result<crate::models::DictionaryItemResponse, Error<CreateDictionaryItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let dictionary_id = params.dictionary_id;
    let item_key = params.item_key;
    let item_value = params.item_value;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/dictionary/{dictionary_id}/item", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), dictionary_id=crate::apis::urlencode(dictionary_id));
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
        let local_var_entity: Option<CreateDictionaryItemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete DictionaryItem given service, dictionary ID, and item key.
pub async fn delete_dictionary_item(configuration: &mut configuration::Configuration, params: DeleteDictionaryItemParams) -> Result<crate::models::InlineResponse200, Error<DeleteDictionaryItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let dictionary_id = params.dictionary_id;
    let dictionary_item_key = params.dictionary_item_key;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/dictionary/{dictionary_id}/item/{dictionary_item_key}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), dictionary_id=crate::apis::urlencode(dictionary_id), dictionary_item_key=crate::apis::urlencode(dictionary_item_key));
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
        let local_var_entity: Option<DeleteDictionaryItemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a single DictionaryItem given service, dictionary ID and item key.
pub async fn get_dictionary_item(configuration: &mut configuration::Configuration, params: GetDictionaryItemParams) -> Result<crate::models::DictionaryItemResponse, Error<GetDictionaryItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let dictionary_id = params.dictionary_id;
    let dictionary_item_key = params.dictionary_item_key;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/dictionary/{dictionary_id}/item/{dictionary_item_key}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), dictionary_id=crate::apis::urlencode(dictionary_id), dictionary_item_key=crate::apis::urlencode(dictionary_item_key));
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
        let local_var_entity: Option<GetDictionaryItemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List of DictionaryItems given service and dictionary ID.
pub async fn list_dictionary_items(configuration: &mut configuration::Configuration, params: ListDictionaryItemsParams) -> Result<Vec<crate::models::DictionaryItemResponse>, Error<ListDictionaryItemsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let dictionary_id = params.dictionary_id;
    let page = params.page;
    let per_page = params.per_page;
    let sort = params.sort;
    let direction = params.direction;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/dictionary/{dictionary_id}/items", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), dictionary_id=crate::apis::urlencode(dictionary_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = direction {
        local_var_req_builder = local_var_req_builder.query(&[("direction", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListDictionaryItemsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update DictionaryItem given service, dictionary ID, item key, and item value.
pub async fn update_dictionary_item(configuration: &mut configuration::Configuration, params: UpdateDictionaryItemParams) -> Result<crate::models::DictionaryItemResponse, Error<UpdateDictionaryItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let dictionary_id = params.dictionary_id;
    let dictionary_item_key = params.dictionary_item_key;
    let item_key = params.item_key;
    let item_value = params.item_value;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/dictionary/{dictionary_id}/item/{dictionary_item_key}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), dictionary_id=crate::apis::urlencode(dictionary_id), dictionary_item_key=crate::apis::urlencode(dictionary_item_key));
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
        let local_var_entity: Option<UpdateDictionaryItemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Upsert DictionaryItem given service, dictionary ID, item key, and item value.
pub async fn upsert_dictionary_item(configuration: &mut configuration::Configuration, params: UpsertDictionaryItemParams) -> Result<crate::models::DictionaryItemResponse, Error<UpsertDictionaryItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let dictionary_id = params.dictionary_id;
    let dictionary_item_key = params.dictionary_item_key;
    let item_key = params.item_key;
    let item_value = params.item_value;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/dictionary/{dictionary_id}/item/{dictionary_item_key}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), dictionary_id=crate::apis::urlencode(dictionary_id), dictionary_item_key=crate::apis::urlencode(dictionary_item_key));
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
        let local_var_entity: Option<UpsertDictionaryItemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

