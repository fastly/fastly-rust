/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_waf_firewall_rule_status`]
#[derive(Clone, Debug, Default)]
pub struct GetWafFirewallRuleStatusParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Firewall.
    pub firewall_id: String,
    /// Alphanumeric string identifying a WAF rule.
    pub waf_rule_id: String
}

/// struct for passing parameters to the method [`list_waf_firewall_rule_statuses`]
#[derive(Clone, Debug, Default)]
pub struct ListWafFirewallRuleStatusesParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Firewall.
    pub firewall_id: String,
    /// Limit results to rule statuses with the specified status.
    pub filter_status: Option<String>,
    /// Limit results to rule statuses whose rules have the specified message.
    pub filter_rule_message: Option<String>,
    /// Limit results to rule statuses whose rules represent the specified ModSecurity rule_id.
    pub filter_rule_rule_id: Option<String>,
    /// Limit results to rule statuses whose rules relate to the specified tag IDs.
    pub filter_rule_tags: Option<String>,
    /// Limit results to rule statuses whose rules related to the named tags.
    pub filter_rule_tags_name: Option<String>,
    /// Include relationships. Optional, comma separated values. Permitted values: `tags`. 
    pub include: Option<String>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>
}

/// struct for passing parameters to the method [`update_waf_firewall_rule_status`]
#[derive(Clone, Debug, Default)]
pub struct UpdateWafFirewallRuleStatusParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Firewall.
    pub firewall_id: String,
    /// Alphanumeric string identifying a WAF rule.
    pub waf_rule_id: String,
    pub request_body: Option<::std::collections::HashMap<String, serde_json::Value>>
}

/// struct for passing parameters to the method [`update_waf_firewall_rule_statuses_tag`]
#[derive(Clone, Debug, Default)]
pub struct UpdateWafFirewallRuleStatusesTagParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a Firewall.
    pub firewall_id: String,
    /// The tag name to use to determine the set of rules to update. For example, OWASP or language-php.
    pub name: Option<String>,
    /// Whether or not to update rule statuses for disabled rules. Optional.
    pub force: Option<String>,
    pub request_body: Option<::std::collections::HashMap<String, serde_json::Value>>
}


/// struct for typed errors of method [`get_waf_firewall_rule_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWafFirewallRuleStatusError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_waf_firewall_rule_statuses`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWafFirewallRuleStatusesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_waf_firewall_rule_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWafFirewallRuleStatusError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_waf_firewall_rule_statuses_tag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWafFirewallRuleStatusesTagError {
    UnknownValue(serde_json::Value),
}


/// Get a specific rule status object for a particular service, firewall, and rule.
pub async fn get_waf_firewall_rule_status(configuration: &mut configuration::Configuration, params: GetWafFirewallRuleStatusParams) -> Result<serde_json::Value, Error<GetWafFirewallRuleStatusError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let firewall_id = params.firewall_id;
    let waf_rule_id = params.waf_rule_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/wafs/{firewall_id}/rules/{waf_rule_id}/rule_status", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), firewall_id=crate::apis::urlencode(firewall_id), waf_rule_id=crate::apis::urlencode(waf_rule_id));
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
        let local_var_entity: Option<GetWafFirewallRuleStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all rule statuses for a particular service and firewall.
pub async fn list_waf_firewall_rule_statuses(configuration: &mut configuration::Configuration, params: ListWafFirewallRuleStatusesParams) -> Result<serde_json::Value, Error<ListWafFirewallRuleStatusesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let firewall_id = params.firewall_id;
    let filter_status = params.filter_status;
    let filter_rule_message = params.filter_rule_message;
    let filter_rule_rule_id = params.filter_rule_rule_id;
    let filter_rule_tags = params.filter_rule_tags;
    let filter_rule_tags_name = params.filter_rule_tags_name;
    let include = params.include;
    let page_number = params.page_number;
    let page_size = params.page_size;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/wafs/{firewall_id}/rule_statuses", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), firewall_id=crate::apis::urlencode(firewall_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_status {
        local_var_req_builder = local_var_req_builder.query(&[("filter[status]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_rule_message {
        local_var_req_builder = local_var_req_builder.query(&[("filter[rule][message]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_rule_rule_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[rule][rule_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_rule_tags {
        local_var_req_builder = local_var_req_builder.query(&[("filter[rule][tags]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_rule_tags_name {
        local_var_req_builder = local_var_req_builder.query(&[("filter[rule][tags][name]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListWafFirewallRuleStatusesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a rule status for a particular service, firewall, and rule.
pub async fn update_waf_firewall_rule_status(configuration: &mut configuration::Configuration, params: UpdateWafFirewallRuleStatusParams) -> Result<serde_json::Value, Error<UpdateWafFirewallRuleStatusError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let firewall_id = params.firewall_id;
    let waf_rule_id = params.waf_rule_id;
    let request_body = params.request_body;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/wafs/{firewall_id}/rules/{waf_rule_id}/rule_status", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), firewall_id=crate::apis::urlencode(firewall_id), waf_rule_id=crate::apis::urlencode(waf_rule_id));
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
    local_var_req_builder = local_var_req_builder.json(&request_body);

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
        let local_var_entity: Option<UpdateWafFirewallRuleStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create or update all rule statuses for a particular service and firewall, based on tag name. By default, only rule status for enabled rules (with status log or block) will be updated. To update rule statuses for disabled rules under the specified tag, use the force attribute.
pub async fn update_waf_firewall_rule_statuses_tag(configuration: &mut configuration::Configuration, params: UpdateWafFirewallRuleStatusesTagParams) -> Result<serde_json::Value, Error<UpdateWafFirewallRuleStatusesTagError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let firewall_id = params.firewall_id;
    let name = params.name;
    let force = params.force;
    let request_body = params.request_body;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/wafs/{firewall_id}/rule_statuses", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), firewall_id=crate::apis::urlencode(firewall_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
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
    local_var_req_builder = local_var_req_builder.json(&request_body);

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
        let local_var_entity: Option<UpdateWafFirewallRuleStatusesTagError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

