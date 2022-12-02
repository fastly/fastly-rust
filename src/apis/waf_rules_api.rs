/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_waf_rule`]
#[derive(Clone, Debug, Default)]
pub struct GetWafRuleParams {
    /// Alphanumeric string identifying a WAF rule.
    pub waf_rule_id: String,
    /// Include relationships. Optional, comma-separated values. Permitted values: `waf_tags` and `waf_rule_revisions`. 
    pub include: Option<String>
}

/// struct for passing parameters to the method [`list_waf_rules`]
#[derive(Clone, Debug, Default)]
pub struct ListWafRulesParams {
    /// Limit the returned rules to a specific ModSecurity rule ID.
    pub filter_modsec_rule_id: Option<String>,
    /// Limit the returned rules to a set linked to a tag by name.
    pub filter_waf_tags_name: Option<String>,
    /// Limit the returned rules to a set linked to a source.
    pub filter_waf_rule_revisions_source: Option<String>,
    /// Limit the returned rules to a set not included in the active firewall version for a firewall.
    pub filter_waf_firewall_id_not_match: Option<String>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>,
    /// Include relationships. Optional, comma-separated values. Permitted values: `waf_tags` and `waf_rule_revisions`. 
    pub include: Option<String>
}


/// struct for typed errors of method [`get_waf_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWafRuleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_waf_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWafRulesError {
    UnknownValue(serde_json::Value),
}


/// Get a specific rule. The `id` provided can be the ModSecurity Rule ID or the Fastly generated rule ID.
pub async fn get_waf_rule(configuration: &mut configuration::Configuration, params: GetWafRuleParams) -> Result<crate::models::WafRuleResponse, Error<GetWafRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let waf_rule_id = params.waf_rule_id;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/rules/{waf_rule_id}", local_var_configuration.base_path, waf_rule_id=crate::apis::urlencode(waf_rule_id));
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
        let local_var_entity: Option<GetWafRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all available WAF rules.
pub async fn list_waf_rules(configuration: &mut configuration::Configuration, params: ListWafRulesParams) -> Result<crate::models::WafRulesResponse, Error<ListWafRulesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let filter_modsec_rule_id = params.filter_modsec_rule_id;
    let filter_waf_tags_name = params.filter_waf_tags_name;
    let filter_waf_rule_revisions_source = params.filter_waf_rule_revisions_source;
    let filter_waf_firewall_id_not_match = params.filter_waf_firewall_id_not_match;
    let page_number = params.page_number;
    let page_size = params.page_size;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/rules", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_modsec_rule_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[modsec_rule_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_waf_tags_name {
        local_var_req_builder = local_var_req_builder.query(&[("filter[waf_tags][name]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_waf_rule_revisions_source {
        local_var_req_builder = local_var_req_builder.query(&[("filter[waf_rule_revisions][source]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_waf_firewall_id_not_match {
        local_var_req_builder = local_var_req_builder.query(&[("filter[waf_firewall.id][not][match]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListWafRulesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

