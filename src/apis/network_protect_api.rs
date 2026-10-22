/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_network_protect_endpoint`]
#[derive(Clone, Debug, Default)]
pub struct CreateNetworkProtectEndpointParams {
    pub network_protect_endpoint_create: crate::models::NetworkProtectEndpointCreate
}

/// struct for passing parameters to the method [`create_network_protect_network`]
#[derive(Clone, Debug, Default)]
pub struct CreateNetworkProtectNetworkParams {
    pub network_protect_network_create: crate::models::NetworkProtectNetworkCreate
}

/// struct for passing parameters to the method [`create_network_protect_prefix_list`]
#[derive(Clone, Debug, Default)]
pub struct CreateNetworkProtectPrefixListParams {
    pub network_protect_prefix_list_create: crate::models::NetworkProtectPrefixListCreate
}

/// struct for passing parameters to the method [`create_network_protect_rule_set`]
#[derive(Clone, Debug, Default)]
pub struct CreateNetworkProtectRuleSetParams {
    pub network_protect_rule_set_create: crate::models::NetworkProtectRuleSetCreate
}

/// struct for passing parameters to the method [`delete_network_protect_endpoint`]
#[derive(Clone, Debug, Default)]
pub struct DeleteNetworkProtectEndpointParams {
    /// Unique identifier of the Network Protect endpoint.
    pub endpoint_id: String
}

/// struct for passing parameters to the method [`delete_network_protect_network`]
#[derive(Clone, Debug, Default)]
pub struct DeleteNetworkProtectNetworkParams {
    /// Unique identifier of the Network Protect network.
    pub network_id: String
}

/// struct for passing parameters to the method [`delete_network_protect_prefix_list`]
#[derive(Clone, Debug, Default)]
pub struct DeleteNetworkProtectPrefixListParams {
    /// Unique identifier of the Network Protect prefix list.
    pub prefix_list_id: String
}

/// struct for passing parameters to the method [`delete_network_protect_rule_set`]
#[derive(Clone, Debug, Default)]
pub struct DeleteNetworkProtectRuleSetParams {
    /// Unique identifier of the Network Protect rule set.
    pub rule_set_id: String
}

/// struct for passing parameters to the method [`get_network_protect_endpoint`]
#[derive(Clone, Debug, Default)]
pub struct GetNetworkProtectEndpointParams {
    /// Unique identifier of the Network Protect endpoint.
    pub endpoint_id: String
}

/// struct for passing parameters to the method [`get_network_protect_network`]
#[derive(Clone, Debug, Default)]
pub struct GetNetworkProtectNetworkParams {
    /// Unique identifier of the Network Protect network.
    pub network_id: String
}

/// struct for passing parameters to the method [`get_network_protect_prefix_list`]
#[derive(Clone, Debug, Default)]
pub struct GetNetworkProtectPrefixListParams {
    /// Unique identifier of the Network Protect prefix list.
    pub prefix_list_id: String
}

/// struct for passing parameters to the method [`get_network_protect_rule_set`]
#[derive(Clone, Debug, Default)]
pub struct GetNetworkProtectRuleSetParams {
    /// Unique identifier of the Network Protect rule set.
    pub rule_set_id: String
}

/// struct for passing parameters to the method [`list_network_protect_endpoints`]
#[derive(Clone, Debug, Default)]
pub struct ListNetworkProtectEndpointsParams {
    /// Maximum resources to return. Defaults to 100. Positive values above 1000 are accepted and clamped to 1000; the effective value is reported in meta.limit. Zero, negative, and non-integer values return a `400 Bad Request` error.
    pub limit: Option<i64>,
    /// Continuation value from a previous response's meta.next_cursor. Omit this parameter to request the first page.
    pub cursor: Option<String>
}

/// struct for passing parameters to the method [`list_network_protect_networks`]
#[derive(Clone, Debug, Default)]
pub struct ListNetworkProtectNetworksParams {
    /// Maximum resources to return. Defaults to 100. Positive values above 1000 are accepted and clamped to 1000; the effective value is reported in meta.limit. Zero, negative, and non-integer values return a `400 Bad Request` error.
    pub limit: Option<i64>,
    /// Continuation value from a previous response's meta.next_cursor. Omit this parameter to request the first page.
    pub cursor: Option<String>
}

/// struct for passing parameters to the method [`list_network_protect_prefix_lists`]
#[derive(Clone, Debug, Default)]
pub struct ListNetworkProtectPrefixListsParams {
    /// Maximum resources to return. Defaults to 100. Positive values above 1000 are accepted and clamped to 1000; the effective value is reported in meta.limit. Zero, negative, and non-integer values return a `400 Bad Request` error.
    pub limit: Option<i64>,
    /// Continuation value from a previous response's meta.next_cursor. Omit this parameter to request the first page.
    pub cursor: Option<String>
}

/// struct for passing parameters to the method [`list_network_protect_rule_sets`]
#[derive(Clone, Debug, Default)]
pub struct ListNetworkProtectRuleSetsParams {
    /// Maximum resources to return. Defaults to 100. Positive values above 1000 are accepted and clamped to 1000; the effective value is reported in meta.limit. Zero, negative, and non-integer values return a `400 Bad Request` error.
    pub limit: Option<i64>,
    /// Continuation value from a previous response's meta.next_cursor. Omit this parameter to request the first page.
    pub cursor: Option<String>
}

/// struct for passing parameters to the method [`update_network_protect_endpoint`]
#[derive(Clone, Debug, Default)]
pub struct UpdateNetworkProtectEndpointParams {
    /// Unique identifier of the Network Protect endpoint.
    pub endpoint_id: String,
    pub network_protect_endpoint_update: crate::models::NetworkProtectEndpointUpdate
}

/// struct for passing parameters to the method [`update_network_protect_network`]
#[derive(Clone, Debug, Default)]
pub struct UpdateNetworkProtectNetworkParams {
    /// Unique identifier of the Network Protect network.
    pub network_id: String,
    pub network_protect_network_update: crate::models::NetworkProtectNetworkUpdate
}

/// struct for passing parameters to the method [`update_network_protect_prefix_list`]
#[derive(Clone, Debug, Default)]
pub struct UpdateNetworkProtectPrefixListParams {
    /// Unique identifier of the Network Protect prefix list.
    pub prefix_list_id: String,
    pub network_protect_prefix_list_update: crate::models::NetworkProtectPrefixListUpdate
}

/// struct for passing parameters to the method [`update_network_protect_rule_set`]
#[derive(Clone, Debug, Default)]
pub struct UpdateNetworkProtectRuleSetParams {
    /// Unique identifier of the Network Protect rule set.
    pub rule_set_id: String,
    pub network_protect_rule_set_update: crate::models::NetworkProtectRuleSetUpdate
}


/// struct for typed errors of method [`create_network_protect_endpoint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNetworkProtectEndpointError {
    Status400(crate::models::NetworkProtectInvalidRequest),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status409(crate::models::NetworkProtectConflict),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_network_protect_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNetworkProtectNetworkError {
    Status400(crate::models::NetworkProtectInvalidRequest),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status409(crate::models::NetworkProtectConflict),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_network_protect_prefix_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNetworkProtectPrefixListError {
    Status400(crate::models::NetworkProtectInvalidRequest),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status409(crate::models::NetworkProtectConflict),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_network_protect_rule_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNetworkProtectRuleSetError {
    Status400(crate::models::NetworkProtectInvalidRequest),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status409(crate::models::NetworkProtectConflict),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_network_protect_endpoint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNetworkProtectEndpointError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status409(crate::models::NetworkProtectConflict),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_network_protect_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNetworkProtectNetworkError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_network_protect_prefix_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNetworkProtectPrefixListError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status409(crate::models::NetworkProtectConflict),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_network_protect_rule_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNetworkProtectRuleSetError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status409(crate::models::NetworkProtectConflict),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_network_protect_endpoint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNetworkProtectEndpointError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_network_protect_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNetworkProtectNetworkError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_network_protect_prefix_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNetworkProtectPrefixListError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_network_protect_rule_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNetworkProtectRuleSetError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_network_protect_endpoints`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNetworkProtectEndpointsError {
    Status400(crate::models::NetworkProtectInvalidRequest),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_network_protect_networks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNetworkProtectNetworksError {
    Status400(crate::models::NetworkProtectInvalidRequest),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_network_protect_prefix_lists`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNetworkProtectPrefixListsError {
    Status400(crate::models::NetworkProtectInvalidRequest),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_network_protect_rule_sets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNetworkProtectRuleSetsError {
    Status400(crate::models::NetworkProtectInvalidRequest),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_network_protect_endpoint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNetworkProtectEndpointError {
    Status400(crate::models::NetworkProtectInvalidRequest),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status409(crate::models::NetworkProtectConflict),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_network_protect_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNetworkProtectNetworkError {
    Status400(crate::models::NetworkProtectInvalidRequest),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status409(crate::models::NetworkProtectConflict),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_network_protect_prefix_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNetworkProtectPrefixListError {
    Status400(crate::models::NetworkProtectInvalidRequest),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status409(crate::models::NetworkProtectConflict),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_network_protect_rule_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNetworkProtectRuleSetError {
    Status400(crate::models::NetworkProtectInvalidRequest),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status409(crate::models::NetworkProtectConflict),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}


/// Creates a GRE delivery destination. A new endpoint is not attached to a network until it is added to an endpoint policy.
pub async fn create_network_protect_endpoint(configuration: &mut configuration::Configuration, params: CreateNetworkProtectEndpointParams) -> Result<crate::models::NetworkProtectEndpointResponse, Error<CreateNetworkProtectEndpointError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let network_protect_endpoint_create = params.network_protect_endpoint_create;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/endpoints", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&network_protect_endpoint_create);

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
        let local_var_entity: Option<CreateNetworkProtectEndpointError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Registers customer-owned IP space for protection. The network starts with advertisement disabled and Fastly does not announce its prefix. Turn advertisement on after LOA and ROA authorization are ready. The create request cannot include advertise; sending it returns a `400 Bad Request` error.
pub async fn create_network_protect_network(configuration: &mut configuration::Configuration, params: CreateNetworkProtectNetworkParams) -> Result<crate::models::NetworkProtectNetworkResponse, Error<CreateNetworkProtectNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let network_protect_network_create = params.network_protect_network_create;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/networks", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&network_protect_network_create);

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
        let local_var_entity: Option<CreateNetworkProtectNetworkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a typed list of addresses that rules can reference. The family is required and cannot be changed. The addresses list may remain empty.
pub async fn create_network_protect_prefix_list(configuration: &mut configuration::Configuration, params: CreateNetworkProtectPrefixListParams) -> Result<crate::models::NetworkProtectPrefixListResponse, Error<CreateNetworkProtectPrefixListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let network_protect_prefix_list_create = params.network_protect_prefix_list_create;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/prefix-lists", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&network_protect_prefix_list_create);

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
        let local_var_entity: Option<CreateNetworkProtectPrefixListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates an ordered rule set and stores its filtering intent. A rule set has at most 64 rules and uses the default action when no rule matches. Custom attachment and detachment are restricted in this release.
pub async fn create_network_protect_rule_set(configuration: &mut configuration::Configuration, params: CreateNetworkProtectRuleSetParams) -> Result<crate::models::NetworkProtectRuleSetResponse, Error<CreateNetworkProtectRuleSetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let network_protect_rule_set_create = params.network_protect_rule_set_create;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/rule-sets", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&network_protect_rule_set_create);

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
        let local_var_entity: Option<CreateNetworkProtectRuleSetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes an endpoint. The endpoint must first be removed from every network policy; active references return a `409 Conflict` error.
pub async fn delete_network_protect_endpoint(configuration: &mut configuration::Configuration, params: DeleteNetworkProtectEndpointParams) -> Result<(), Error<DeleteNetworkProtectEndpointError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let endpoint_id = params.endpoint_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/endpoints/{endpoint_id}", local_var_configuration.base_path, endpoint_id=crate::apis::urlencode(endpoint_id));
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
        let local_var_entity: Option<DeleteNetworkProtectEndpointError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a network and stops its advertisement and filtering. Endpoints and rule sets used by the network are not deleted.
pub async fn delete_network_protect_network(configuration: &mut configuration::Configuration, params: DeleteNetworkProtectNetworkParams) -> Result<(), Error<DeleteNetworkProtectNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let network_id = params.network_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/networks/{network_id}", local_var_configuration.base_path, network_id=crate::apis::urlencode(network_id));
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
        let local_var_entity: Option<DeleteNetworkProtectNetworkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a prefix list. Active rule references must be removed first; referenced lists return a `409 Conflict` error. Unknown and foreign IDs are indistinguishable and return a `404 Not Found` error.
pub async fn delete_network_protect_prefix_list(configuration: &mut configuration::Configuration, params: DeleteNetworkProtectPrefixListParams) -> Result<(), Error<DeleteNetworkProtectPrefixListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let prefix_list_id = params.prefix_list_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/prefix-lists/{prefix_list_id}", local_var_configuration.base_path, prefix_list_id=crate::apis::urlencode(prefix_list_id));
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
        let local_var_entity: Option<DeleteNetworkProtectPrefixListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a rule set. Attached rule sets cannot be deleted and return a `409 Conflict` error until their references are removed.
pub async fn delete_network_protect_rule_set(configuration: &mut configuration::Configuration, params: DeleteNetworkProtectRuleSetParams) -> Result<(), Error<DeleteNetworkProtectRuleSetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let rule_set_id = params.rule_set_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/rule-sets/{rule_set_id}", local_var_configuration.base_path, rule_set_id=crate::apis::urlencode(rule_set_id));
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
        let local_var_entity: Option<DeleteNetworkProtectRuleSetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a single endpoint by its identifier.
pub async fn get_network_protect_endpoint(configuration: &mut configuration::Configuration, params: GetNetworkProtectEndpointParams) -> Result<crate::models::NetworkProtectEndpointResponse, Error<GetNetworkProtectEndpointError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let endpoint_id = params.endpoint_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/endpoints/{endpoint_id}", local_var_configuration.base_path, endpoint_id=crate::apis::urlencode(endpoint_id));
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
        let local_var_entity: Option<GetNetworkProtectEndpointError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a single network by its identifier.
pub async fn get_network_protect_network(configuration: &mut configuration::Configuration, params: GetNetworkProtectNetworkParams) -> Result<crate::models::NetworkProtectNetworkResponse, Error<GetNetworkProtectNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let network_id = params.network_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/networks/{network_id}", local_var_configuration.base_path, network_id=crate::apis::urlencode(network_id));
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
        let local_var_entity: Option<GetNetworkProtectNetworkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a single prefix list by its identifier. Unknown and foreign IDs are indistinguishable and return a `404 Not Found` error.
pub async fn get_network_protect_prefix_list(configuration: &mut configuration::Configuration, params: GetNetworkProtectPrefixListParams) -> Result<crate::models::NetworkProtectPrefixListResponse, Error<GetNetworkProtectPrefixListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let prefix_list_id = params.prefix_list_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/prefix-lists/{prefix_list_id}", local_var_configuration.base_path, prefix_list_id=crate::apis::urlencode(prefix_list_id));
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
        let local_var_entity: Option<GetNetworkProtectPrefixListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a single rule set with rules in evaluation order.
pub async fn get_network_protect_rule_set(configuration: &mut configuration::Configuration, params: GetNetworkProtectRuleSetParams) -> Result<crate::models::NetworkProtectRuleSetResponse, Error<GetNetworkProtectRuleSetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let rule_set_id = params.rule_set_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/rule-sets/{rule_set_id}", local_var_configuration.base_path, rule_set_id=crate::apis::urlencode(rule_set_id));
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
        let local_var_entity: Option<GetNetworkProtectRuleSetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists the customer's GRE delivery endpoints in ascending ID order. The response is cursor-paginated and includes the effective page limit.
pub async fn list_network_protect_endpoints(configuration: &mut configuration::Configuration, params: ListNetworkProtectEndpointsParams) -> Result<crate::models::NetworkProtectEndpointsResponse, Error<ListNetworkProtectEndpointsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let limit = params.limit;
    let cursor = params.cursor;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/endpoints", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListNetworkProtectEndpointsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists the customer's networks in ascending ID order. The response is cursor-paginated and includes the effective page limit.
pub async fn list_network_protect_networks(configuration: &mut configuration::Configuration, params: ListNetworkProtectNetworksParams) -> Result<crate::models::NetworkProtectNetworksResponse, Error<ListNetworkProtectNetworksError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let limit = params.limit;
    let cursor = params.cursor;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/networks", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListNetworkProtectNetworksError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists prefix lists in ascending ID order. Positive limits above 1000 are accepted and clamped to 1000. Invalid limits return a `400 Bad Request` error.
pub async fn list_network_protect_prefix_lists(configuration: &mut configuration::Configuration, params: ListNetworkProtectPrefixListsParams) -> Result<crate::models::NetworkProtectPrefixListsResponse, Error<ListNetworkProtectPrefixListsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let limit = params.limit;
    let cursor = params.cursor;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/prefix-lists", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListNetworkProtectPrefixListsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists rule sets in ascending ID order. Positive limits above 1000 are accepted and clamped to 1000; invalid limits return a `400 Bad Request` error.
pub async fn list_network_protect_rule_sets(configuration: &mut configuration::Configuration, params: ListNetworkProtectRuleSetsParams) -> Result<crate::models::NetworkProtectRuleSetsResponse, Error<ListNetworkProtectRuleSetsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let limit = params.limit;
    let cursor = params.cursor;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/rule-sets", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListNetworkProtectRuleSetsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates an endpoint. Omitted fields remain unchanged. The IP address cannot be changed; create a new endpoint if the destination changes.
pub async fn update_network_protect_endpoint(configuration: &mut configuration::Configuration, params: UpdateNetworkProtectEndpointParams) -> Result<crate::models::NetworkProtectEndpointResponse, Error<UpdateNetworkProtectEndpointError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let endpoint_id = params.endpoint_id;
    let network_protect_endpoint_update = params.network_protect_endpoint_update;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/endpoints/{endpoint_id}", local_var_configuration.base_path, endpoint_id=crate::apis::urlencode(endpoint_id));
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
    local_var_req_builder = local_var_req_builder.json(&network_protect_endpoint_update);

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
        let local_var_entity: Option<UpdateNetworkProtectEndpointError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a network. Omitted fields remain unchanged. The prefix cannot be changed. Sending `endpoint_policies` replaces the complete policy list; an empty array removes every policy. Advertisement is desired state. Enabling it requires an approved LOA, a valid ROA, and a ready default-region delivery policy. A readiness failure returns a `409 Conflict` error.
pub async fn update_network_protect_network(configuration: &mut configuration::Configuration, params: UpdateNetworkProtectNetworkParams) -> Result<crate::models::NetworkProtectNetworkResponse, Error<UpdateNetworkProtectNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let network_id = params.network_id;
    let network_protect_network_update = params.network_protect_network_update;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/networks/{network_id}", local_var_configuration.base_path, network_id=crate::apis::urlencode(network_id));
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
    local_var_req_builder = local_var_req_builder.json(&network_protect_network_update);

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
        let local_var_entity: Option<UpdateNetworkProtectNetworkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a prefix list. The family cannot be changed. Sending addresses replaces the complete list; an empty array clears it when no active source-list restriction applies. Addresses are canonicalized, deduplicated, and sorted by the backend.
pub async fn update_network_protect_prefix_list(configuration: &mut configuration::Configuration, params: UpdateNetworkProtectPrefixListParams) -> Result<crate::models::NetworkProtectPrefixListResponse, Error<UpdateNetworkProtectPrefixListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let prefix_list_id = params.prefix_list_id;
    let network_protect_prefix_list_update = params.network_protect_prefix_list_update;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/prefix-lists/{prefix_list_id}", local_var_configuration.base_path, prefix_list_id=crate::apis::urlencode(prefix_list_id));
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
    local_var_req_builder = local_var_req_builder.json(&network_protect_prefix_list_update);

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
        let local_var_entity: Option<UpdateNetworkProtectPrefixListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a rule set. Omitted fields remain unchanged. Sending rules replaces the complete ordered list; an empty array removes every rule. Custom attachment and detachment remain restricted in this release.
pub async fn update_network_protect_rule_set(configuration: &mut configuration::Configuration, params: UpdateNetworkProtectRuleSetParams) -> Result<crate::models::NetworkProtectRuleSetResponse, Error<UpdateNetworkProtectRuleSetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let rule_set_id = params.rule_set_id;
    let network_protect_rule_set_update = params.network_protect_rule_set_update;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/network-protect/v1/rule-sets/{rule_set_id}", local_var_configuration.base_path, rule_set_id=crate::apis::urlencode(rule_set_id));
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
    local_var_req_builder = local_var_req_builder.json(&network_protect_rule_set_update);

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
        let local_var_entity: Option<UpdateNetworkProtectRuleSetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

