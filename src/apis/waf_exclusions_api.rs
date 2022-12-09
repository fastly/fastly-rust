/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_waf_rule_exclusion`]
#[derive(Clone, Debug, Default)]
pub struct CreateWafRuleExclusionParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a WAF firewall version.
    pub firewall_version_number: i32,
    pub waf_exclusion: Option<crate::models::WafExclusion>
}

/// struct for passing parameters to the method [`delete_waf_rule_exclusion`]
#[derive(Clone, Debug, Default)]
pub struct DeleteWafRuleExclusionParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a WAF firewall version.
    pub firewall_version_number: i32,
    /// A numeric ID identifying a WAF exclusion.
    pub exclusion_number: i32
}

/// struct for passing parameters to the method [`get_waf_rule_exclusion`]
#[derive(Clone, Debug, Default)]
pub struct GetWafRuleExclusionParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a WAF firewall version.
    pub firewall_version_number: i32,
    /// A numeric ID identifying a WAF exclusion.
    pub exclusion_number: i32
}

/// struct for passing parameters to the method [`list_waf_rule_exclusions`]
#[derive(Clone, Debug, Default)]
pub struct ListWafRuleExclusionsParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a WAF firewall version.
    pub firewall_version_number: i32,
    /// Filters the results based on this exclusion type.
    pub filter_exclusion_type: Option<String>,
    /// Filters the results based on name.
    pub filter_name: Option<String>,
    /// Filters the results based on this ModSecurity rule ID.
    pub filter_waf_rules_modsec_rule_id: Option<i32>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>,
    /// Include relationships. Optional, comma-separated values. Permitted values: `waf_rules` and `waf_rule_revisions`. 
    pub include: Option<String>
}

/// struct for passing parameters to the method [`update_waf_rule_exclusion`]
#[derive(Clone, Debug, Default)]
pub struct UpdateWafRuleExclusionParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a WAF firewall version.
    pub firewall_version_number: i32,
    /// A numeric ID identifying a WAF exclusion.
    pub exclusion_number: i32,
    pub waf_exclusion: Option<crate::models::WafExclusion>
}


/// struct for typed errors of method [`create_waf_rule_exclusion`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWafRuleExclusionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_waf_rule_exclusion`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWafRuleExclusionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_waf_rule_exclusion`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWafRuleExclusionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_waf_rule_exclusions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWafRuleExclusionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_waf_rule_exclusion`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWafRuleExclusionError {
    UnknownValue(serde_json::Value),
}


/// Create a WAF exclusion for a particular firewall version.
pub async fn create_waf_rule_exclusion(configuration: &mut configuration::Configuration, params: CreateWafRuleExclusionParams) -> Result<crate::models::WafExclusionResponse, Error<CreateWafRuleExclusionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let firewall_version_number = params.firewall_version_number;
    let waf_exclusion = params.waf_exclusion;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{firewall_version_number}/exclusions", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), firewall_version_number=firewall_version_number);
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
    local_var_req_builder = local_var_req_builder.json(&waf_exclusion);

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
        let local_var_entity: Option<CreateWafRuleExclusionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a WAF exclusion for a particular firewall version.
pub async fn delete_waf_rule_exclusion(configuration: &mut configuration::Configuration, params: DeleteWafRuleExclusionParams) -> Result<(), Error<DeleteWafRuleExclusionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let firewall_version_number = params.firewall_version_number;
    let exclusion_number = params.exclusion_number;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{firewall_version_number}/exclusions/{exclusion_number}", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), firewall_version_number=firewall_version_number, exclusion_number=exclusion_number);
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
        let local_var_entity: Option<DeleteWafRuleExclusionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a specific WAF exclusion object.
pub async fn get_waf_rule_exclusion(configuration: &mut configuration::Configuration, params: GetWafRuleExclusionParams) -> Result<crate::models::WafExclusionResponse, Error<GetWafRuleExclusionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let firewall_version_number = params.firewall_version_number;
    let exclusion_number = params.exclusion_number;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{firewall_version_number}/exclusions/{exclusion_number}", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), firewall_version_number=firewall_version_number, exclusion_number=exclusion_number);
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
        let local_var_entity: Option<GetWafRuleExclusionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all exclusions for a particular firewall version.
pub async fn list_waf_rule_exclusions(configuration: &mut configuration::Configuration, params: ListWafRuleExclusionsParams) -> Result<crate::models::WafExclusionsResponse, Error<ListWafRuleExclusionsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let firewall_version_number = params.firewall_version_number;
    let filter_exclusion_type = params.filter_exclusion_type;
    let filter_name = params.filter_name;
    let filter_waf_rules_modsec_rule_id = params.filter_waf_rules_modsec_rule_id;
    let page_number = params.page_number;
    let page_size = params.page_size;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{firewall_version_number}/exclusions", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), firewall_version_number=firewall_version_number);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_exclusion_type {
        local_var_req_builder = local_var_req_builder.query(&[("filter[exclusion_type]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_name {
        local_var_req_builder = local_var_req_builder.query(&[("filter[name]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_waf_rules_modsec_rule_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[waf_rules.modsec_rule_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_number {
        local_var_req_builder = local_var_req_builder.query(&[("page[number]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("page[size]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListWafRuleExclusionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a WAF exclusion for a particular firewall version.
pub async fn update_waf_rule_exclusion(configuration: &mut configuration::Configuration, params: UpdateWafRuleExclusionParams) -> Result<crate::models::WafExclusionResponse, Error<UpdateWafRuleExclusionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let firewall_version_number = params.firewall_version_number;
    let exclusion_number = params.exclusion_number;
    let waf_exclusion = params.waf_exclusion;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{firewall_version_number}/exclusions/{exclusion_number}", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), firewall_version_number=firewall_version_number, exclusion_number=exclusion_number);
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
    local_var_req_builder = local_var_req_builder.json(&waf_exclusion);

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
        let local_var_entity: Option<UpdateWafRuleExclusionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

