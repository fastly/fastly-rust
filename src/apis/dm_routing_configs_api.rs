/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`activate_dm_routing_config_draft`]
#[derive(Clone, Debug, Default)]
pub struct ActivateDmRoutingConfigDraftParams {
    pub config_id: String
}

/// struct for passing parameters to the method [`create_dm_routing_config`]
#[derive(Clone, Debug, Default)]
pub struct CreateDmRoutingConfigParams {
    pub routing_config: Option<crate::models::RoutingConfig>
}

/// struct for passing parameters to the method [`create_dm_routing_config_path`]
#[derive(Clone, Debug, Default)]
pub struct CreateDmRoutingConfigPathParams {
    pub config_id: String,
    pub path_create: Option<crate::models::PathCreate>
}

/// struct for passing parameters to the method [`create_dm_routing_config_rule`]
#[derive(Clone, Debug, Default)]
pub struct CreateDmRoutingConfigRuleParams {
    pub config_id: String,
    pub path_id: String,
    pub rule_create: Option<crate::models::RuleCreate>
}

/// struct for passing parameters to the method [`deactivate_dm_routing_config`]
#[derive(Clone, Debug, Default)]
pub struct DeactivateDmRoutingConfigParams {
    pub config_id: String
}

/// struct for passing parameters to the method [`delete_dm_routing_config`]
#[derive(Clone, Debug, Default)]
pub struct DeleteDmRoutingConfigParams {
    pub config_id: String,
    /// When `true`, allows deleting a routing config that has an active version. This is destructive — traffic routing for any paths served by the config will stop immediately.
    pub force: Option<bool>
}

/// struct for passing parameters to the method [`delete_dm_routing_config_inactive_versions`]
#[derive(Clone, Debug, Default)]
pub struct DeleteDmRoutingConfigInactiveVersionsParams {
    pub config_id: String
}

/// struct for passing parameters to the method [`delete_dm_routing_config_path`]
#[derive(Clone, Debug, Default)]
pub struct DeleteDmRoutingConfigPathParams {
    pub config_id: String,
    pub path_id: String
}

/// struct for passing parameters to the method [`delete_dm_routing_config_rule`]
#[derive(Clone, Debug, Default)]
pub struct DeleteDmRoutingConfigRuleParams {
    pub config_id: String,
    pub path_id: String,
    pub rule_id: String
}

/// struct for passing parameters to the method [`discard_dm_routing_config_draft`]
#[derive(Clone, Debug, Default)]
pub struct DiscardDmRoutingConfigDraftParams {
    pub config_id: String
}

/// struct for passing parameters to the method [`get_dm_routing_config`]
#[derive(Clone, Debug, Default)]
pub struct GetDmRoutingConfigParams {
    pub config_id: String
}

/// struct for passing parameters to the method [`get_dm_routing_config_draft_diff`]
#[derive(Clone, Debug, Default)]
pub struct GetDmRoutingConfigDraftDiffParams {
    pub config_id: String
}

/// struct for passing parameters to the method [`get_dm_routing_config_path`]
#[derive(Clone, Debug, Default)]
pub struct GetDmRoutingConfigPathParams {
    pub config_id: String,
    pub path_id: String
}

/// struct for passing parameters to the method [`get_dm_routing_config_rule`]
#[derive(Clone, Debug, Default)]
pub struct GetDmRoutingConfigRuleParams {
    pub config_id: String,
    pub path_id: String,
    pub rule_id: String
}

/// struct for passing parameters to the method [`list_dm_routing_config_paths`]
#[derive(Clone, Debug, Default)]
pub struct ListDmRoutingConfigPathsParams {
    pub config_id: String,
    /// Filter results by path pattern. The match strategy is controlled by the `match` parameter.
    pub path: Option<String>,
    /// How to match the value of the `path` filter against existing path patterns. Has no effect unless `path` is also provided.
    pub _match: Option<String>,
    /// The order in which to list the results.
    pub sort: Option<String>,
    /// Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty.
    pub cursor: Option<String>,
    /// Limit how many results are returned.
    pub limit: Option<i32>
}

/// struct for passing parameters to the method [`list_dm_routing_config_rules`]
#[derive(Clone, Debug, Default)]
pub struct ListDmRoutingConfigRulesParams {
    pub config_id: String,
    pub path_id: String,
    /// The order in which to list the results.
    pub sort: Option<String>,
    /// Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty.
    pub cursor: Option<String>,
    /// Limit how many results are returned.
    pub limit: Option<i32>
}

/// struct for passing parameters to the method [`list_dm_routing_config_versions`]
#[derive(Clone, Debug, Default)]
pub struct ListDmRoutingConfigVersionsParams {
    pub config_id: String,
    /// The order in which to list the results.
    pub sort: Option<String>,
    /// Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty.
    pub cursor: Option<String>,
    /// Limit how many results are returned.
    pub limit: Option<i32>
}

/// struct for passing parameters to the method [`list_dm_routing_configs`]
#[derive(Clone, Debug, Default)]
pub struct ListDmRoutingConfigsParams {
    /// Filter configs by lifecycle state. Accepts a comma-separated list of state values (e.g. `?state=active,active-with-draft`). Returns only configs whose current state matches one of the provided values. Returns 400 if any value is not a recognised state.
    pub state: Option<Vec<String>>,
    /// The order in which to list the results.
    pub sort: Option<String>,
    /// Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty.
    pub cursor: Option<String>,
    /// Limit how many results are returned.
    pub limit: Option<i32>
}

/// struct for passing parameters to the method [`reactivate_dm_routing_config_version`]
#[derive(Clone, Debug, Default)]
pub struct ReactivateDmRoutingConfigVersionParams {
    pub config_id: String,
    pub version_id: String
}

/// struct for passing parameters to the method [`update_dm_routing_config_draft`]
#[derive(Clone, Debug, Default)]
pub struct UpdateDmRoutingConfigDraftParams {
    pub config_id: String,
    pub draft_update: Option<crate::models::DraftUpdate>
}

/// struct for passing parameters to the method [`update_dm_routing_config_path`]
#[derive(Clone, Debug, Default)]
pub struct UpdateDmRoutingConfigPathParams {
    pub config_id: String,
    pub path_id: String,
    pub path_update: Option<crate::models::PathUpdate>
}

/// struct for passing parameters to the method [`update_dm_routing_config_rule`]
#[derive(Clone, Debug, Default)]
pub struct UpdateDmRoutingConfigRuleParams {
    pub config_id: String,
    pub path_id: String,
    pub rule_id: String,
    pub rule_update: Option<crate::models::RuleUpdate>
}


/// struct for typed errors of method [`activate_dm_routing_config_draft`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivateDmRoutingConfigDraftError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_dm_routing_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDmRoutingConfigError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_dm_routing_config_path`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDmRoutingConfigPathError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_dm_routing_config_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDmRoutingConfigRuleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deactivate_dm_routing_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeactivateDmRoutingConfigError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_dm_routing_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDmRoutingConfigError {
    Status409(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_dm_routing_config_inactive_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDmRoutingConfigInactiveVersionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_dm_routing_config_path`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDmRoutingConfigPathError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_dm_routing_config_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDmRoutingConfigRuleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`discard_dm_routing_config_draft`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiscardDmRoutingConfigDraftError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dm_routing_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDmRoutingConfigError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dm_routing_config_draft_diff`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDmRoutingConfigDraftDiffError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dm_routing_config_path`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDmRoutingConfigPathError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dm_routing_config_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDmRoutingConfigRuleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_dm_routing_config_paths`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDmRoutingConfigPathsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_dm_routing_config_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDmRoutingConfigRulesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_dm_routing_config_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDmRoutingConfigVersionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_dm_routing_configs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDmRoutingConfigsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`reactivate_dm_routing_config_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReactivateDmRoutingConfigVersionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_dm_routing_config_draft`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDmRoutingConfigDraftError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_dm_routing_config_path`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDmRoutingConfigPathError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_dm_routing_config_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDmRoutingConfigRuleError {
    UnknownValue(serde_json::Value),
}


/// Activate the current draft version. The previously active version, if any, becomes inactive but is retained in version history.
pub async fn activate_dm_routing_config_draft(configuration: &mut configuration::Configuration, params: ActivateDmRoutingConfigDraftParams) -> Result<crate::models::RoutingConfigVersionResponse, Error<ActivateDmRoutingConfigDraftError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/activate", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id));
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
        let local_var_entity: Option<ActivateDmRoutingConfigDraftError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new routing config. An optional `initial_version` may be provided to seed the config with paths and rules in a single request, and may also be activated immediately.
pub async fn create_dm_routing_config(configuration: &mut configuration::Configuration, params: CreateDmRoutingConfigParams) -> Result<crate::models::RoutingConfigResponse, Error<CreateDmRoutingConfigError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let routing_config = params.routing_config;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&routing_config);

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
        let local_var_entity: Option<CreateDmRoutingConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Add a new path to the config's draft version. If no draft exists, one is created automatically by cloning the active version.
pub async fn create_dm_routing_config_path(configuration: &mut configuration::Configuration, params: CreateDmRoutingConfigPathParams) -> Result<crate::models::PathResponse, Error<CreateDmRoutingConfigPathError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let path_create = params.path_create;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/paths", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id));
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
    local_var_req_builder = local_var_req_builder.json(&path_create);

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
        let local_var_entity: Option<CreateDmRoutingConfigPathError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Add a new rule to a path on the config's draft version. If no draft exists, one is created automatically by cloning the active version. A rule with an empty `conditions` array is a default (catch-all) rule and there can be at most one default rule per path.
pub async fn create_dm_routing_config_rule(configuration: &mut configuration::Configuration, params: CreateDmRoutingConfigRuleParams) -> Result<crate::models::RuleResponse, Error<CreateDmRoutingConfigRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let path_id = params.path_id;
    let rule_create = params.rule_create;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/paths/{path_id}/rules", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id), path_id=crate::apis::urlencode(path_id));
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
    local_var_req_builder = local_var_req_builder.json(&rule_create);

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
        let local_var_entity: Option<CreateDmRoutingConfigRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Clear the active version designation. This is a bookkeeping operation only — it does not stop edge traffic. Minerva continues serving the last-activated version until the domain association is removed in Spotless. Only removing the routing config from the domain (via Spotless) triggers Neptune to drop the reference, which causes Minerva to stop fetching and eventually clean up the cached config. Idempotent: returns 200 even if already deactivated.
pub async fn deactivate_dm_routing_config(configuration: &mut configuration::Configuration, params: DeactivateDmRoutingConfigParams) -> Result<crate::models::RoutingConfigResponse, Error<DeactivateDmRoutingConfigError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/deactivate", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id));
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
        let local_var_entity: Option<DeactivateDmRoutingConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a routing config. By default, configs that have an active version cannot be deleted. Pass `force=true` to bypass the active-version check — this is destructive and will immediately stop traffic routing for any paths the config serves. The `force` parameter does **not** bypass the domain-association check; if domains are still associated, deletion is rejected with 409 regardless of `force`.
pub async fn delete_dm_routing_config(configuration: &mut configuration::Configuration, params: DeleteDmRoutingConfigParams) -> Result<(), Error<DeleteDmRoutingConfigError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let force = params.force;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = force {
        local_var_req_builder = local_var_req_builder.query(&[("force", &local_var_str.to_string())]);
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
        let local_var_entity: Option<DeleteDmRoutingConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete all inactive versions for a routing config. The currently active version, if any, is retained.
pub async fn delete_dm_routing_config_inactive_versions(configuration: &mut configuration::Configuration, params: DeleteDmRoutingConfigInactiveVersionsParams) -> Result<(), Error<DeleteDmRoutingConfigInactiveVersionsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/versions/inactive", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id));
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
        let local_var_entity: Option<DeleteDmRoutingConfigInactiveVersionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a path from the config's draft version. If no draft exists, one is created automatically by cloning the active version.
pub async fn delete_dm_routing_config_path(configuration: &mut configuration::Configuration, params: DeleteDmRoutingConfigPathParams) -> Result<(), Error<DeleteDmRoutingConfigPathError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let path_id = params.path_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/paths/{path_id}", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id), path_id=crate::apis::urlencode(path_id));
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
        let local_var_entity: Option<DeleteDmRoutingConfigPathError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a rule from the config's draft version. If no draft exists, one is created automatically by cloning the active version.
pub async fn delete_dm_routing_config_rule(configuration: &mut configuration::Configuration, params: DeleteDmRoutingConfigRuleParams) -> Result<(), Error<DeleteDmRoutingConfigRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let path_id = params.path_id;
    let rule_id = params.rule_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/paths/{path_id}/rules/{rule_id}", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id), path_id=crate::apis::urlencode(path_id), rule_id=crate::apis::urlencode(rule_id));
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
        let local_var_entity: Option<DeleteDmRoutingConfigRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the current draft version, reverting any unactivated changes.
pub async fn discard_dm_routing_config_draft(configuration: &mut configuration::Configuration, params: DiscardDmRoutingConfigDraftParams) -> Result<(), Error<DiscardDmRoutingConfigDraftError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/draft", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id));
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
        let local_var_entity: Option<DiscardDmRoutingConfigDraftError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a single routing config by its identifier.
pub async fn get_dm_routing_config(configuration: &mut configuration::Configuration, params: GetDmRoutingConfigParams) -> Result<crate::models::RoutingConfigResponse, Error<GetDmRoutingConfigError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id));
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
        let local_var_entity: Option<GetDmRoutingConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Compare the current draft version against the active version and return the paths and rules that have been added, modified, or deleted.
pub async fn get_dm_routing_config_draft_diff(configuration: &mut configuration::Configuration, params: GetDmRoutingConfigDraftDiffParams) -> Result<crate::models::DraftDiff, Error<GetDmRoutingConfigDraftDiffError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/draft/diff", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id));
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
        let local_var_entity: Option<GetDmRoutingConfigDraftDiffError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a single path by its stable identifier.
pub async fn get_dm_routing_config_path(configuration: &mut configuration::Configuration, params: GetDmRoutingConfigPathParams) -> Result<crate::models::PathResponse, Error<GetDmRoutingConfigPathError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let path_id = params.path_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/paths/{path_id}", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id), path_id=crate::apis::urlencode(path_id));
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
        let local_var_entity: Option<GetDmRoutingConfigPathError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a single rule by its stable identifier.
pub async fn get_dm_routing_config_rule(configuration: &mut configuration::Configuration, params: GetDmRoutingConfigRuleParams) -> Result<crate::models::RuleResponse, Error<GetDmRoutingConfigRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let path_id = params.path_id;
    let rule_id = params.rule_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/paths/{path_id}/rules/{rule_id}", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id), path_id=crate::apis::urlencode(path_id), rule_id=crate::apis::urlencode(rule_id));
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
        let local_var_entity: Option<GetDmRoutingConfigRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List paths for the config. Returns paths from the active version if one exists, otherwise from the draft.
pub async fn list_dm_routing_config_paths(configuration: &mut configuration::Configuration, params: ListDmRoutingConfigPathsParams) -> Result<crate::models::PathsResponse, Error<ListDmRoutingConfigPathsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let path = params.path;
    let _match = params._match;
    let sort = params.sort;
    let cursor = params.cursor;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/paths", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = path {
        local_var_req_builder = local_var_req_builder.query(&[("path", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = _match {
        local_var_req_builder = local_var_req_builder.query(&[("match", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListDmRoutingConfigPathsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all rules for a path in evaluation order.
pub async fn list_dm_routing_config_rules(configuration: &mut configuration::Configuration, params: ListDmRoutingConfigRulesParams) -> Result<crate::models::RulesResponse, Error<ListDmRoutingConfigRulesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let path_id = params.path_id;
    let sort = params.sort;
    let cursor = params.cursor;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/paths/{path_id}/rules", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id), path_id=crate::apis::urlencode(path_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListDmRoutingConfigRulesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all versions for a routing config.
pub async fn list_dm_routing_config_versions(configuration: &mut configuration::Configuration, params: ListDmRoutingConfigVersionsParams) -> Result<crate::models::VersionsResponse, Error<ListDmRoutingConfigVersionsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let sort = params.sort;
    let cursor = params.cursor;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/versions", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListDmRoutingConfigVersionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all routing configs for the authenticated customer.
pub async fn list_dm_routing_configs(configuration: &mut configuration::Configuration, params: ListDmRoutingConfigsParams) -> Result<crate::models::RoutingConfigsResponse, Error<ListDmRoutingConfigsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let state = params.state;
    let sort = params.sort;
    let cursor = params.cursor;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = state {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("state".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("state", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListDmRoutingConfigsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Reactivate a previously-active version. The currently active version, if any, becomes inactive but is retained in version history.
pub async fn reactivate_dm_routing_config_version(configuration: &mut configuration::Configuration, params: ReactivateDmRoutingConfigVersionParams) -> Result<crate::models::RoutingConfigVersionResponse, Error<ReactivateDmRoutingConfigVersionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/versions/{version_id}/activate", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id), version_id=crate::apis::urlencode(version_id));
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
        let local_var_entity: Option<ReactivateDmRoutingConfigVersionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update metadata on the draft version, such as its comment. If no draft exists, one is created automatically by cloning the active version.
pub async fn update_dm_routing_config_draft(configuration: &mut configuration::Configuration, params: UpdateDmRoutingConfigDraftParams) -> Result<crate::models::RoutingConfigVersionResponse, Error<UpdateDmRoutingConfigDraftError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let draft_update = params.draft_update;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/draft", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id));
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
    local_var_req_builder = local_var_req_builder.json(&draft_update);

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
        let local_var_entity: Option<UpdateDmRoutingConfigDraftError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a path on the config's draft version. If no draft exists, one is created automatically by cloning the active version.
pub async fn update_dm_routing_config_path(configuration: &mut configuration::Configuration, params: UpdateDmRoutingConfigPathParams) -> Result<crate::models::PathResponse, Error<UpdateDmRoutingConfigPathError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let path_id = params.path_id;
    let path_update = params.path_update;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/paths/{path_id}", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id), path_id=crate::apis::urlencode(path_id));
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
    local_var_req_builder = local_var_req_builder.json(&path_update);

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
        let local_var_entity: Option<UpdateDmRoutingConfigPathError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a rule on the config's draft version. If no draft exists, one is created automatically by cloning the active version.
pub async fn update_dm_routing_config_rule(configuration: &mut configuration::Configuration, params: UpdateDmRoutingConfigRuleParams) -> Result<crate::models::RuleResponse, Error<UpdateDmRoutingConfigRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let config_id = params.config_id;
    let path_id = params.path_id;
    let rule_id = params.rule_id;
    let rule_update = params.rule_update;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/routing-configs/{config_id}/paths/{path_id}/rules/{rule_id}", local_var_configuration.base_path, config_id=crate::apis::urlencode(config_id), path_id=crate::apis::urlencode(path_id), rule_id=crate::apis::urlencode(rule_id));
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
    local_var_req_builder = local_var_req_builder.json(&rule_update);

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
        let local_var_entity: Option<UpdateDmRoutingConfigRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

