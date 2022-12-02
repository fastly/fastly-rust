/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`clone_waf_firewall_version`]
#[derive(Clone, Debug, Default)]
pub struct CloneWafFirewallVersionParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a WAF firewall version.
    pub firewall_version_number: i32
}

/// struct for passing parameters to the method [`create_waf_firewall_version`]
#[derive(Clone, Debug, Default)]
pub struct CreateWafFirewallVersionParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    pub waf_firewall_version: Option<crate::models::WafFirewallVersion>
}

/// struct for passing parameters to the method [`deploy_activate_waf_firewall_version`]
#[derive(Clone, Debug, Default)]
pub struct DeployActivateWafFirewallVersionParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a WAF firewall version.
    pub firewall_version_number: i32
}

/// struct for passing parameters to the method [`get_waf_firewall_version`]
#[derive(Clone, Debug, Default)]
pub struct GetWafFirewallVersionParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a WAF firewall version.
    pub firewall_version_number: i32,
    /// Include relationships. Optional, comma-separated values. Permitted values: `waf_firewall` and `waf_active_rules`. 
    pub include: Option<String>
}

/// struct for passing parameters to the method [`list_waf_firewall_versions`]
#[derive(Clone, Debug, Default)]
pub struct ListWafFirewallVersionsParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Include relationships. Optional.
    pub include: Option<String>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>
}

/// struct for passing parameters to the method [`update_waf_firewall_version`]
#[derive(Clone, Debug, Default)]
pub struct UpdateWafFirewallVersionParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a WAF firewall version.
    pub firewall_version_number: i32,
    pub waf_firewall_version: Option<crate::models::WafFirewallVersion>
}


/// struct for typed errors of method [`clone_waf_firewall_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloneWafFirewallVersionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_waf_firewall_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWafFirewallVersionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deploy_activate_waf_firewall_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeployActivateWafFirewallVersionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_waf_firewall_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWafFirewallVersionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_waf_firewall_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWafFirewallVersionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_waf_firewall_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWafFirewallVersionError {
    UnknownValue(serde_json::Value),
}


/// Clone a specific, existing firewall version into a new, draft firewall version.
pub async fn clone_waf_firewall_version(configuration: &mut configuration::Configuration, params: CloneWafFirewallVersionParams) -> Result<crate::models::WafFirewallVersionResponse, Error<CloneWafFirewallVersionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let firewall_version_number = params.firewall_version_number;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{firewall_version_number}/clone", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), firewall_version_number=firewall_version_number);
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
        let local_var_entity: Option<CloneWafFirewallVersionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new, draft firewall version.
pub async fn create_waf_firewall_version(configuration: &mut configuration::Configuration, params: CreateWafFirewallVersionParams) -> Result<crate::models::WafFirewallVersionResponse, Error<CreateWafFirewallVersionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let waf_firewall_version = params.waf_firewall_version;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id));
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
    local_var_req_builder = local_var_req_builder.json(&waf_firewall_version);

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
        let local_var_entity: Option<CreateWafFirewallVersionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deploy or activate a specific firewall version. If a firewall has been disabled, deploying a firewall version will automatically enable the firewall again.
pub async fn deploy_activate_waf_firewall_version(configuration: &mut configuration::Configuration, params: DeployActivateWafFirewallVersionParams) -> Result<serde_json::Value, Error<DeployActivateWafFirewallVersionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let firewall_version_number = params.firewall_version_number;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{firewall_version_number}/activate", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), firewall_version_number=firewall_version_number);
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
        let local_var_entity: Option<DeployActivateWafFirewallVersionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get details about a specific firewall version.
pub async fn get_waf_firewall_version(configuration: &mut configuration::Configuration, params: GetWafFirewallVersionParams) -> Result<crate::models::WafFirewallVersionResponse, Error<GetWafFirewallVersionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let firewall_version_number = params.firewall_version_number;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{firewall_version_number}", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), firewall_version_number=firewall_version_number);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include {
        local_var_req_builder = local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetWafFirewallVersionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a list of firewall versions associated with a specific firewall.
pub async fn list_waf_firewall_versions(configuration: &mut configuration::Configuration, params: ListWafFirewallVersionsParams) -> Result<crate::models::WafFirewallVersionsResponse, Error<ListWafFirewallVersionsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let include = params.include;
    let page_number = params.page_number;
    let page_size = params.page_size;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include {
        local_var_req_builder = local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_number {
        local_var_req_builder = local_var_req_builder.query(&[("page[number]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("page[size]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListWafFirewallVersionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a specific firewall version.
pub async fn update_waf_firewall_version(configuration: &mut configuration::Configuration, params: UpdateWafFirewallVersionParams) -> Result<crate::models::WafFirewallVersionResponse, Error<UpdateWafFirewallVersionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let firewall_version_number = params.firewall_version_number;
    let waf_firewall_version = params.waf_firewall_version;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{firewall_version_number}", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), firewall_version_number=firewall_version_number);
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
    local_var_req_builder = local_var_req_builder.json(&waf_firewall_version);

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
        let local_var_entity: Option<UpdateWafFirewallVersionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

