/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_waf_firewall`]
#[derive(Clone, Debug, Default)]
pub struct CreateWafFirewallParams {
    pub waf_firewall: Option<crate::models::WafFirewall>
}

/// struct for passing parameters to the method [`delete_waf_firewall`]
#[derive(Clone, Debug, Default)]
pub struct DeleteWafFirewallParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    pub waf_firewall: Option<crate::models::WafFirewall>
}

/// struct for passing parameters to the method [`get_waf_firewall`]
#[derive(Clone, Debug, Default)]
pub struct GetWafFirewallParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Limit the results returned to a specific service version.
    pub filter_service_version_number: Option<String>,
    /// Include related objects. Optional.
    pub include: Option<String>
}

/// struct for passing parameters to the method [`list_waf_firewalls`]
#[derive(Clone, Debug, Default)]
pub struct ListWafFirewallsParams {
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>,
    /// Limit the results returned to a specific service.
    pub filter_service_id: Option<String>,
    /// Limit the results returned to a specific service version.
    pub filter_service_version_number: Option<String>,
    /// Include related objects. Optional.
    pub include: Option<String>
}

/// struct for passing parameters to the method [`update_waf_firewall`]
#[derive(Clone, Debug, Default)]
pub struct UpdateWafFirewallParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    pub waf_firewall: Option<crate::models::WafFirewall>
}


/// struct for typed errors of method [`create_waf_firewall`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWafFirewallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_waf_firewall`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWafFirewallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_waf_firewall`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWafFirewallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_waf_firewalls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWafFirewallsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_waf_firewall`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWafFirewallError {
    UnknownValue(serde_json::Value),
}


/// Create a firewall object for a particular service and service version using a defined `prefetch_condition` and `response`. If the `prefetch_condition` or the `response` is missing from the request body, Fastly will generate a default object on your service. 
pub async fn create_waf_firewall(configuration: &mut configuration::Configuration, params: CreateWafFirewallParams) -> Result<crate::models::WafFirewallResponse, Error<CreateWafFirewallError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let waf_firewall = params.waf_firewall;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&waf_firewall);

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
        let local_var_entity: Option<CreateWafFirewallError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the firewall object for a particular service and service version. 
pub async fn delete_waf_firewall(configuration: &mut configuration::Configuration, params: DeleteWafFirewallParams) -> Result<(), Error<DeleteWafFirewallError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let waf_firewall = params.waf_firewall;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id));
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
    local_var_req_builder = local_var_req_builder.json(&waf_firewall);

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
        let local_var_entity: Option<DeleteWafFirewallError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a specific firewall object.
pub async fn get_waf_firewall(configuration: &mut configuration::Configuration, params: GetWafFirewallParams) -> Result<crate::models::WafFirewallResponse, Error<GetWafFirewallError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let filter_service_version_number = params.filter_service_version_number;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_service_version_number {
        local_var_req_builder = local_var_req_builder.query(&[("filter[service_version_number]", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<GetWafFirewallError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all firewall objects.
pub async fn list_waf_firewalls(configuration: &mut configuration::Configuration, params: ListWafFirewallsParams) -> Result<crate::models::WafFirewallsResponse, Error<ListWafFirewallsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_number = params.page_number;
    let page_size = params.page_size;
    let filter_service_id = params.filter_service_id;
    let filter_service_version_number = params.filter_service_version_number;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_number {
        local_var_req_builder = local_var_req_builder.query(&[("page[number]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("page[size]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_service_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[service_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_service_version_number {
        local_var_req_builder = local_var_req_builder.query(&[("filter[service_version_number]", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<ListWafFirewallsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a firewall object for a particular service and service version. Specifying a `service_version_number` is required. 
pub async fn update_waf_firewall(configuration: &mut configuration::Configuration, params: UpdateWafFirewallParams) -> Result<crate::models::WafFirewallResponse, Error<UpdateWafFirewallError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let waf_firewall = params.waf_firewall;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id));
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
    local_var_req_builder = local_var_req_builder.json(&waf_firewall);

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
        let local_var_entity: Option<UpdateWafFirewallError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

