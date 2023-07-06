/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_legacy_waf_firewall_rule_vcl`]
#[derive(Clone, Debug, Default)]
pub struct GetLegacyWafFirewallRuleVclParams {
    /// Alphanumeric string identifying a Firewall.
    pub firewall_id: String,
    /// Alphanumeric string identifying a WAF rule.
    pub waf_rule_id: String
}

/// struct for passing parameters to the method [`get_legacy_waf_rule`]
#[derive(Clone, Debug, Default)]
pub struct GetLegacyWafRuleParams {
    /// Alphanumeric string identifying a WAF rule.
    pub waf_rule_id: String,
    /// Optional. Limit rule to a specific configuration set or pass \"all\" to search all configuration sets, including stale ones.
    pub filter_configuration_set_id: Option<String>,
    /// Include relationships. Optional. Comma separated values. Permitted values: `tags`, `rule_statuses`, `source`, and `vcl`. 
    pub include: Option<String>
}

/// struct for passing parameters to the method [`get_legacy_waf_rule_vcl`]
#[derive(Clone, Debug, Default)]
pub struct GetLegacyWafRuleVclParams {
    /// Alphanumeric string identifying a WAF rule.
    pub waf_rule_id: String
}

/// struct for passing parameters to the method [`list_legacy_waf_rules`]
#[derive(Clone, Debug, Default)]
pub struct ListLegacyWafRulesParams {
    /// Limit the returned rules to a specific rule ID.
    pub filter_rule_id: Option<String>,
    /// Limit the returned rules to a specific severity.
    pub filter_severity: Option<String>,
    /// Limit the returned rules to a set linked to a tag by name.
    pub filter_tags_name: Option<String>,
    /// Optional. Limit rules to specific configuration set or pass \"all\" to search all configuration sets, including stale ones.
    pub filter_configuration_set_id: Option<String>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>,
    /// Include relationships. Optional. Comma separated values. Permitted values: `tags`, `rule_statuses`, and `source`. 
    pub include: Option<String>
}


/// struct for typed errors of method [`get_legacy_waf_firewall_rule_vcl`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLegacyWafFirewallRuleVclError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_legacy_waf_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLegacyWafRuleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_legacy_waf_rule_vcl`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLegacyWafRuleVclError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_legacy_waf_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLegacyWafRulesError {
    UnknownValue(serde_json::Value),
}


/// Get associated VCL for a specific rule associated with a specific firewall.
pub async fn get_legacy_waf_firewall_rule_vcl(configuration: &mut configuration::Configuration, params: GetLegacyWafFirewallRuleVclParams) -> Result<serde_json::Value, Error<GetLegacyWafFirewallRuleVclError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let waf_rule_id = params.waf_rule_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/wafs/{firewall_id}/rules/{waf_rule_id}/vcl", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), waf_rule_id=crate::apis::urlencode(waf_rule_id));
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
        let local_var_entity: Option<GetLegacyWafFirewallRuleVclError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a specific rule.
pub async fn get_legacy_waf_rule(configuration: &mut configuration::Configuration, params: GetLegacyWafRuleParams) -> Result<serde_json::Value, Error<GetLegacyWafRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let waf_rule_id = params.waf_rule_id;
    let filter_configuration_set_id = params.filter_configuration_set_id;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/wafs/rules/{waf_rule_id}", local_var_configuration.base_path, waf_rule_id=crate::apis::urlencode(waf_rule_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_configuration_set_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[configuration_set_id]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetLegacyWafRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get associated VCL for a specific rule.
pub async fn get_legacy_waf_rule_vcl(configuration: &mut configuration::Configuration, params: GetLegacyWafRuleVclParams) -> Result<serde_json::Value, Error<GetLegacyWafRuleVclError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let waf_rule_id = params.waf_rule_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/wafs/rules/{waf_rule_id}/vcl", local_var_configuration.base_path, waf_rule_id=crate::apis::urlencode(waf_rule_id));
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
        let local_var_entity: Option<GetLegacyWafRuleVclError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all rules in the latest configuration set.
pub async fn list_legacy_waf_rules(configuration: &mut configuration::Configuration, params: ListLegacyWafRulesParams) -> Result<Vec<serde_json::Value>, Error<ListLegacyWafRulesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let filter_rule_id = params.filter_rule_id;
    let filter_severity = params.filter_severity;
    let filter_tags_name = params.filter_tags_name;
    let filter_configuration_set_id = params.filter_configuration_set_id;
    let page_number = params.page_number;
    let page_size = params.page_size;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/wafs/rules", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_rule_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[rule_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_severity {
        local_var_req_builder = local_var_req_builder.query(&[("filter[severity]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_tags_name {
        local_var_req_builder = local_var_req_builder.query(&[("filter[tags][name]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_configuration_set_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[configuration_set_id]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListLegacyWafRulesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

