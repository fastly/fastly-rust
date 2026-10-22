/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`bot_management_get_bot`]
#[derive(Clone, Debug, Default)]
pub struct BotManagementGetBotParams {
    /// The ID of the bot management workspace.
    pub workspace_id: String,
    /// The ID of the bot category.
    pub category_id: String,
    /// The ID of the bot.
    pub bot_id: String
}

/// struct for passing parameters to the method [`bot_management_get_category`]
#[derive(Clone, Debug, Default)]
pub struct BotManagementGetCategoryParams {
    /// The ID of the bot management workspace.
    pub workspace_id: String,
    /// The ID of the bot category.
    pub category_id: String
}

/// struct for passing parameters to the method [`bot_management_get_workspace`]
#[derive(Clone, Debug, Default)]
pub struct BotManagementGetWorkspaceParams {
    /// The ID of the bot management workspace.
    pub workspace_id: String
}

/// struct for passing parameters to the method [`bot_management_list_bots`]
#[derive(Clone, Debug, Default)]
pub struct BotManagementListBotsParams {
    /// The ID of the bot management workspace.
    pub workspace_id: String,
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Cursor for pagination. Use the `next_cursor` value from a previous response to request the next page of results.
    pub cursor: Option<String>
}

/// struct for passing parameters to the method [`bot_management_list_categories`]
#[derive(Clone, Debug, Default)]
pub struct BotManagementListCategoriesParams {
    /// The ID of the bot management workspace.
    pub workspace_id: String,
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Cursor for pagination. Use the `next_cursor` value from a previous response to request the next page of results.
    pub cursor: Option<String>
}

/// struct for passing parameters to the method [`bot_management_list_category_bots`]
#[derive(Clone, Debug, Default)]
pub struct BotManagementListCategoryBotsParams {
    /// The ID of the bot management workspace.
    pub workspace_id: String,
    /// The ID of the bot category.
    pub category_id: String,
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Cursor for pagination. Use the `next_cursor` value from a previous response to request the next page of results.
    pub cursor: Option<String>
}

/// struct for passing parameters to the method [`bot_management_list_workspaces`]
#[derive(Clone, Debug, Default)]
pub struct BotManagementListWorkspacesParams {
    /// Filter results to the workspace attached to this service. Returns an empty list if the service is not attached to a workspace.
    pub service_id: Option<String>,
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Cursor for pagination. Use the `next_cursor` value from a previous response to request the next page of results.
    pub cursor: Option<String>
}

/// struct for passing parameters to the method [`bot_management_update_bot`]
#[derive(Clone, Debug, Default)]
pub struct BotManagementUpdateBotParams {
    /// The ID of the bot management workspace.
    pub workspace_id: String,
    /// The ID of the bot category.
    pub category_id: String,
    /// The ID of the bot.
    pub bot_id: String,
    pub bot_management_bot: crate::models::BotManagementBot
}

/// struct for passing parameters to the method [`bot_management_update_category`]
#[derive(Clone, Debug, Default)]
pub struct BotManagementUpdateCategoryParams {
    /// The ID of the bot management workspace.
    pub workspace_id: String,
    /// The ID of the bot category.
    pub category_id: String,
    pub bot_management_category: crate::models::BotManagementCategory
}

/// struct for passing parameters to the method [`bot_management_update_workspace`]
#[derive(Clone, Debug, Default)]
pub struct BotManagementUpdateWorkspaceParams {
    /// The ID of the bot management workspace.
    pub workspace_id: String,
    pub bot_management_workspace: crate::models::BotManagementWorkspace
}


/// struct for typed errors of method [`bot_management_get_bot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotManagementGetBotError {
    Status400(crate::models::BotManagementInvalidRequestError),
    Status401(crate::models::BotManagementNotAuthenticatedError),
    Status403(crate::models::BotManagementNotAllowedError),
    Status404(crate::models::BotManagementNotFoundError),
    Status429(crate::models::BotManagementRateLimitError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bot_management_get_category`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotManagementGetCategoryError {
    Status400(crate::models::BotManagementInvalidRequestError),
    Status401(crate::models::BotManagementNotAuthenticatedError),
    Status403(crate::models::BotManagementNotAllowedError),
    Status404(crate::models::BotManagementNotFoundError),
    Status429(crate::models::BotManagementRateLimitError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bot_management_get_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotManagementGetWorkspaceError {
    Status400(crate::models::BotManagementInvalidRequestError),
    Status401(crate::models::BotManagementNotAuthenticatedError),
    Status403(crate::models::BotManagementNotAllowedError),
    Status404(crate::models::BotManagementNotFoundError),
    Status429(crate::models::BotManagementRateLimitError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bot_management_list_bots`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotManagementListBotsError {
    Status400(crate::models::BotManagementInvalidRequestError),
    Status401(crate::models::BotManagementNotAuthenticatedError),
    Status403(crate::models::BotManagementNotAllowedError),
    Status404(crate::models::BotManagementNotFoundError),
    Status429(crate::models::BotManagementRateLimitError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bot_management_list_categories`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotManagementListCategoriesError {
    Status400(crate::models::BotManagementInvalidRequestError),
    Status401(crate::models::BotManagementNotAuthenticatedError),
    Status403(crate::models::BotManagementNotAllowedError),
    Status404(crate::models::BotManagementNotFoundError),
    Status429(crate::models::BotManagementRateLimitError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bot_management_list_category_bots`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotManagementListCategoryBotsError {
    Status400(crate::models::BotManagementInvalidRequestError),
    Status401(crate::models::BotManagementNotAuthenticatedError),
    Status403(crate::models::BotManagementNotAllowedError),
    Status404(crate::models::BotManagementNotFoundError),
    Status429(crate::models::BotManagementRateLimitError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bot_management_list_workspaces`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotManagementListWorkspacesError {
    Status400(crate::models::BotManagementInvalidRequestError),
    Status401(crate::models::BotManagementNotAuthenticatedError),
    Status403(crate::models::BotManagementNotAllowedError),
    Status429(crate::models::BotManagementRateLimitError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bot_management_update_bot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotManagementUpdateBotError {
    Status400(crate::models::BotManagementInvalidRequestError),
    Status401(crate::models::BotManagementNotAuthenticatedError),
    Status403(crate::models::BotManagementNotAllowedError),
    Status404(crate::models::BotManagementNotFoundError),
    Status429(crate::models::BotManagementRateLimitError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bot_management_update_category`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotManagementUpdateCategoryError {
    Status400(crate::models::BotManagementInvalidRequestError),
    Status401(crate::models::BotManagementNotAuthenticatedError),
    Status403(crate::models::BotManagementNotAllowedError),
    Status404(crate::models::BotManagementNotFoundError),
    Status429(crate::models::BotManagementRateLimitError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bot_management_update_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotManagementUpdateWorkspaceError {
    Status400(crate::models::BotManagementInvalidRequestError),
    Status401(crate::models::BotManagementNotAuthenticatedError),
    Status403(crate::models::BotManagementNotAllowedError),
    Status404(crate::models::BotManagementNotFoundError),
    Status429(crate::models::BotManagementRateLimitError),
    UnknownValue(serde_json::Value),
}


/// Get the attributes of a bot in a category.
pub async fn bot_management_get_bot(configuration: &mut configuration::Configuration, params: BotManagementGetBotParams) -> Result<crate::models::BotManagementBotResponse, Error<BotManagementGetBotError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let workspace_id = params.workspace_id;
    let category_id = params.category_id;
    let bot_id = params.bot_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/bot-management/v1/workspaces/{workspace_id}/policy/categories/{category_id}/bots/{bot_id}", local_var_configuration.base_path, workspace_id=crate::apis::urlencode(workspace_id), category_id=crate::apis::urlencode(category_id), bot_id=crate::apis::urlencode(bot_id));
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
        let local_var_entity: Option<BotManagementGetBotError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the attributes of a bot category.
pub async fn bot_management_get_category(configuration: &mut configuration::Configuration, params: BotManagementGetCategoryParams) -> Result<crate::models::BotManagementCategoryResponse, Error<BotManagementGetCategoryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let workspace_id = params.workspace_id;
    let category_id = params.category_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/bot-management/v1/workspaces/{workspace_id}/policy/categories/{category_id}", local_var_configuration.base_path, workspace_id=crate::apis::urlencode(workspace_id), category_id=crate::apis::urlencode(category_id));
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
        let local_var_entity: Option<BotManagementGetCategoryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a bot management workspace.
pub async fn bot_management_get_workspace(configuration: &mut configuration::Configuration, params: BotManagementGetWorkspaceParams) -> Result<crate::models::BotManagementWorkspaceResponse, Error<BotManagementGetWorkspaceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let workspace_id = params.workspace_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/bot-management/v1/workspaces/{workspace_id}", local_var_configuration.base_path, workspace_id=crate::apis::urlencode(workspace_id));
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
        let local_var_entity: Option<BotManagementGetWorkspaceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all bots in a workspace's policy across all categories.
pub async fn bot_management_list_bots(configuration: &mut configuration::Configuration, params: BotManagementListBotsParams) -> Result<crate::models::BotManagementWorkspaceBotsResponse, Error<BotManagementListBotsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let workspace_id = params.workspace_id;
    let limit = params.limit;
    let cursor = params.cursor;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/bot-management/v1/workspaces/{workspace_id}/policy/bots", local_var_configuration.base_path, workspace_id=crate::apis::urlencode(workspace_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
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
        let local_var_entity: Option<BotManagementListBotsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all bot categories in a workspace's policy. Each category has an action that applies to every bot in the category. To list the bots themselves, use [list all bots in a workspace](#bot-management-list-bots) or [list bots in a category](#bot-management-list-category-bots).
pub async fn bot_management_list_categories(configuration: &mut configuration::Configuration, params: BotManagementListCategoriesParams) -> Result<crate::models::BotManagementCategoriesResponse, Error<BotManagementListCategoriesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let workspace_id = params.workspace_id;
    let limit = params.limit;
    let cursor = params.cursor;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/bot-management/v1/workspaces/{workspace_id}/policy/categories", local_var_configuration.base_path, workspace_id=crate::apis::urlencode(workspace_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
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
        let local_var_entity: Option<BotManagementListCategoriesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all bots in a bot category.
pub async fn bot_management_list_category_bots(configuration: &mut configuration::Configuration, params: BotManagementListCategoryBotsParams) -> Result<crate::models::BotManagementBotsResponse, Error<BotManagementListCategoryBotsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let workspace_id = params.workspace_id;
    let category_id = params.category_id;
    let limit = params.limit;
    let cursor = params.cursor;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/bot-management/v1/workspaces/{workspace_id}/policy/categories/{category_id}/bots", local_var_configuration.base_path, workspace_id=crate::apis::urlencode(workspace_id), category_id=crate::apis::urlencode(category_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
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
        let local_var_entity: Option<BotManagementListCategoryBotsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List bot management workspaces.
pub async fn bot_management_list_workspaces(configuration: &mut configuration::Configuration, params: BotManagementListWorkspacesParams) -> Result<crate::models::BotManagementWorkspacesResponse, Error<BotManagementListWorkspacesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let limit = params.limit;
    let cursor = params.cursor;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/bot-management/v1/workspaces", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = service_id {
        local_var_req_builder = local_var_req_builder.query(&[("service_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
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
        let local_var_entity: Option<BotManagementListWorkspacesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Set the action for a bot. Set the action to 'inherit' to apply the category's action or to a specific value to override the category's action.
pub async fn bot_management_update_bot(configuration: &mut configuration::Configuration, params: BotManagementUpdateBotParams) -> Result<crate::models::BotManagementBotResponse, Error<BotManagementUpdateBotError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let workspace_id = params.workspace_id;
    let category_id = params.category_id;
    let bot_id = params.bot_id;
    let bot_management_bot = params.bot_management_bot;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/bot-management/v1/workspaces/{workspace_id}/policy/categories/{category_id}/bots/{bot_id}", local_var_configuration.base_path, workspace_id=crate::apis::urlencode(workspace_id), category_id=crate::apis::urlencode(category_id), bot_id=crate::apis::urlencode(bot_id));
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
    local_var_req_builder = local_var_req_builder.json(&bot_management_bot);

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
        let local_var_entity: Option<BotManagementUpdateBotError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Set the action for a bot category. Every bot in the category configured with the 'inherit' action uses the action set for the category.
pub async fn bot_management_update_category(configuration: &mut configuration::Configuration, params: BotManagementUpdateCategoryParams) -> Result<crate::models::BotManagementCategoryResponse, Error<BotManagementUpdateCategoryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let workspace_id = params.workspace_id;
    let category_id = params.category_id;
    let bot_management_category = params.bot_management_category;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/bot-management/v1/workspaces/{workspace_id}/policy/categories/{category_id}", local_var_configuration.base_path, workspace_id=crate::apis::urlencode(workspace_id), category_id=crate::apis::urlencode(category_id));
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
    local_var_req_builder = local_var_req_builder.json(&bot_management_category);

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
        let local_var_entity: Option<BotManagementUpdateCategoryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the attributes of a bot management workspace.
pub async fn bot_management_update_workspace(configuration: &mut configuration::Configuration, params: BotManagementUpdateWorkspaceParams) -> Result<crate::models::BotManagementWorkspaceResponse, Error<BotManagementUpdateWorkspaceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let workspace_id = params.workspace_id;
    let bot_management_workspace = params.bot_management_workspace;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/bot-management/v1/workspaces/{workspace_id}", local_var_configuration.base_path, workspace_id=crate::apis::urlencode(workspace_id));
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
    local_var_req_builder = local_var_req_builder.json(&bot_management_workspace);

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
        let local_var_entity: Option<BotManagementUpdateWorkspaceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

