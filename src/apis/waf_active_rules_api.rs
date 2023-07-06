/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`bulk_delete_waf_active_rules`]
#[derive(Clone, Debug, Default)]
pub struct BulkDeleteWafActiveRulesParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    pub request_body: Option<::std::collections::HashMap<String, serde_json::Value>>
}

/// struct for passing parameters to the method [`bulk_update_waf_active_rules`]
#[derive(Clone, Debug, Default)]
pub struct BulkUpdateWafActiveRulesParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    pub body: Option<crate::models::WafActiveRuleData>
}

/// struct for passing parameters to the method [`create_waf_active_rule`]
#[derive(Clone, Debug, Default)]
pub struct CreateWafActiveRuleParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    pub waf_active_rule: Option<crate::models::WafActiveRule>
}

/// struct for passing parameters to the method [`create_waf_active_rules_tag`]
#[derive(Clone, Debug, Default)]
pub struct CreateWafActiveRulesTagParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// Name of the tag.
    pub waf_tag_name: String,
    pub waf_active_rule: Option<crate::models::WafActiveRule>
}

/// struct for passing parameters to the method [`delete_waf_active_rule`]
#[derive(Clone, Debug, Default)]
pub struct DeleteWafActiveRuleParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// Alphanumeric string identifying a WAF rule.
    pub waf_rule_id: String
}

/// struct for passing parameters to the method [`get_waf_active_rule`]
#[derive(Clone, Debug, Default)]
pub struct GetWafActiveRuleParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// Alphanumeric string identifying a WAF rule.
    pub waf_rule_id: String,
    /// Include relationships. Optional, comma-separated values. Permitted values: `waf_rule_revision` and `waf_firewall_version`. 
    pub include: Option<String>
}

/// struct for passing parameters to the method [`list_waf_active_rules`]
#[derive(Clone, Debug, Default)]
pub struct ListWafActiveRulesParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// Limit results to active rules with the specified status.
    pub filter_status: Option<String>,
    /// Limit results to active rules with the specified message.
    pub filter_waf_rule_revision_message: Option<String>,
    /// Limit results to active rules that represent the specified ModSecurity modsec_rule_id.
    pub filter_waf_rule_revision_modsec_rule_id: Option<String>,
    /// Limit results to active rules referencing an outdated rule revision.
    pub filter_outdated: Option<String>,
    /// Include relationships. Optional, comma-separated values. Permitted values: `waf_rule_revision` and `waf_firewall_version`. 
    pub include: Option<String>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>
}

/// struct for passing parameters to the method [`update_waf_active_rule`]
#[derive(Clone, Debug, Default)]
pub struct UpdateWafActiveRuleParams {
    /// Alphanumeric string identifying a WAF Firewall.
    pub firewall_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// Alphanumeric string identifying a WAF rule.
    pub waf_rule_id: String,
    pub waf_active_rule: Option<crate::models::WafActiveRule>
}


/// struct for typed errors of method [`bulk_delete_waf_active_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkDeleteWafActiveRulesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_update_waf_active_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateWafActiveRulesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_waf_active_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWafActiveRuleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_waf_active_rules_tag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWafActiveRulesTagError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_waf_active_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWafActiveRuleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_waf_active_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWafActiveRuleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_waf_active_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWafActiveRulesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_waf_active_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWafActiveRuleError {
    UnknownValue(serde_json::Value),
}


/// Delete many active rules on a particular firewall version using the active rule ID. Limited to 500 rules per request.
pub async fn bulk_delete_waf_active_rules(configuration: &mut configuration::Configuration, params: BulkDeleteWafActiveRulesParams) -> Result<(), Error<BulkDeleteWafActiveRulesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let version_id = params.version_id;
    let request_body = params.request_body;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{version_id}/active-rules", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), version_id=version_id);
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
    local_var_req_builder = local_var_req_builder.json(&request_body);

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
        let local_var_entity: Option<BulkDeleteWafActiveRulesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Bulk update all active rules on a [firewall version](https://developer.fastly.com/reference/api/waf/firewall-version/). This endpoint will not add new active rules, only update existing active rules.
pub async fn bulk_update_waf_active_rules(configuration: &mut configuration::Configuration, params: BulkUpdateWafActiveRulesParams) -> Result<(), Error<BulkUpdateWafActiveRulesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let version_id = params.version_id;
    let body = params.body;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{version_id}/active-rules/bulk", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), version_id=version_id);
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
    local_var_req_builder = local_var_req_builder.json(&body);

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
        Ok(())
    } else {
        let local_var_entity: Option<BulkUpdateWafActiveRulesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create an active rule for a particular firewall version.
pub async fn create_waf_active_rule(configuration: &mut configuration::Configuration, params: CreateWafActiveRuleParams) -> Result<crate::models::WafActiveRuleCreationResponse, Error<CreateWafActiveRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let version_id = params.version_id;
    let waf_active_rule = params.waf_active_rule;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{version_id}/active-rules", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), version_id=version_id);
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
    local_var_req_builder = local_var_req_builder.json(&waf_active_rule);

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
        let local_var_entity: Option<CreateWafActiveRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create active rules by tag. This endpoint will create active rules using the latest revision available for each rule.
pub async fn create_waf_active_rules_tag(configuration: &mut configuration::Configuration, params: CreateWafActiveRulesTagParams) -> Result<(), Error<CreateWafActiveRulesTagError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let version_id = params.version_id;
    let waf_tag_name = params.waf_tag_name;
    let waf_active_rule = params.waf_active_rule;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{version_id}/tags/{waf_tag_name}/active-rules", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), version_id=version_id, waf_tag_name=crate::apis::urlencode(waf_tag_name));
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
    local_var_req_builder = local_var_req_builder.json(&waf_active_rule);

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
        Ok(())
    } else {
        let local_var_entity: Option<CreateWafActiveRulesTagError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an active rule for a particular firewall version.
pub async fn delete_waf_active_rule(configuration: &mut configuration::Configuration, params: DeleteWafActiveRuleParams) -> Result<(), Error<DeleteWafActiveRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let version_id = params.version_id;
    let waf_rule_id = params.waf_rule_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{version_id}/active-rules/{waf_rule_id}", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), version_id=version_id, waf_rule_id=crate::apis::urlencode(waf_rule_id));
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
        let local_var_entity: Option<DeleteWafActiveRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a specific active rule object. Includes details of the rule revision associated with the active rule object by default.
pub async fn get_waf_active_rule(configuration: &mut configuration::Configuration, params: GetWafActiveRuleParams) -> Result<crate::models::WafActiveRuleResponse, Error<GetWafActiveRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let version_id = params.version_id;
    let waf_rule_id = params.waf_rule_id;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{version_id}/active-rules/{waf_rule_id}", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), version_id=version_id, waf_rule_id=crate::apis::urlencode(waf_rule_id));
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
        let local_var_entity: Option<GetWafActiveRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all active rules for a particular firewall version.
pub async fn list_waf_active_rules(configuration: &mut configuration::Configuration, params: ListWafActiveRulesParams) -> Result<crate::models::WafActiveRulesResponse, Error<ListWafActiveRulesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let version_id = params.version_id;
    let filter_status = params.filter_status;
    let filter_waf_rule_revision_message = params.filter_waf_rule_revision_message;
    let filter_waf_rule_revision_modsec_rule_id = params.filter_waf_rule_revision_modsec_rule_id;
    let filter_outdated = params.filter_outdated;
    let include = params.include;
    let page_number = params.page_number;
    let page_size = params.page_size;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{version_id}/active-rules", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), version_id=version_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_status {
        local_var_req_builder = local_var_req_builder.query(&[("filter[status]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_waf_rule_revision_message {
        local_var_req_builder = local_var_req_builder.query(&[("filter[waf_rule_revision][message]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_waf_rule_revision_modsec_rule_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[waf_rule_revision][modsec_rule_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_outdated {
        local_var_req_builder = local_var_req_builder.query(&[("filter[outdated]", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<ListWafActiveRulesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update an active rule's status for a particular firewall version.
pub async fn update_waf_active_rule(configuration: &mut configuration::Configuration, params: UpdateWafActiveRuleParams) -> Result<crate::models::WafActiveRuleResponse, Error<UpdateWafActiveRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let firewall_id = params.firewall_id;
    let version_id = params.version_id;
    let waf_rule_id = params.waf_rule_id;
    let waf_active_rule = params.waf_active_rule;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/firewalls/{firewall_id}/versions/{version_id}/active-rules/{waf_rule_id}", local_var_configuration.base_path, firewall_id=crate::apis::urlencode(firewall_id), version_id=version_id, waf_rule_id=crate::apis::urlencode(waf_rule_id));
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
    local_var_req_builder = local_var_req_builder.json(&waf_active_rule);

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
        let local_var_entity: Option<UpdateWafActiveRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

