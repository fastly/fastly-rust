/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`ddos_protection_event_get`]
#[derive(Clone, Debug, Default)]
pub struct DdosProtectionEventGetParams {
    /// Unique ID of the event.
    pub event_id: String
}

/// struct for passing parameters to the method [`ddos_protection_event_list`]
#[derive(Clone, Debug, Default)]
pub struct DdosProtectionEventListParams {
    /// Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty.
    pub cursor: Option<String>,
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Filter results based on a service_id.
    pub service_id: Option<String>,
    /// Represents the start of a date-time range expressed in RFC 3339 format.
    pub from: Option<String>,
    /// Represents the end of a date-time range expressed in RFC 3339 format.
    pub to: Option<String>,
    pub name: Option<String>
}

/// struct for passing parameters to the method [`ddos_protection_event_rule_list`]
#[derive(Clone, Debug, Default)]
pub struct DdosProtectionEventRuleListParams {
    /// Unique ID of the event.
    pub event_id: String,
    /// Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty.
    pub cursor: Option<String>,
    /// Limit how many results are returned.
    pub limit: Option<i32>
}

/// struct for passing parameters to the method [`ddos_protection_rule_get`]
#[derive(Clone, Debug, Default)]
pub struct DdosProtectionRuleGetParams {
    /// Unique ID of the rule.
    pub rule_id: String
}

/// struct for passing parameters to the method [`ddos_protection_rule_patch`]
#[derive(Clone, Debug, Default)]
pub struct DdosProtectionRulePatchParams {
    /// Unique ID of the rule.
    pub rule_id: String,
    pub ddos_protection_rule_patch: Option<crate::models::DdosProtectionRulePatch>
}

/// struct for passing parameters to the method [`ddos_protection_traffic_stats_rule_get`]
#[derive(Clone, Debug, Default)]
pub struct DdosProtectionTrafficStatsRuleGetParams {
    /// Unique ID of the event.
    pub event_id: String,
    /// Unique ID of the rule.
    pub rule_id: String
}


/// struct for typed errors of method [`ddos_protection_event_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DdosProtectionEventGetError {
    Status401(crate::models::DdosProtectionError),
    Status404(crate::models::DdosProtectionError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ddos_protection_event_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DdosProtectionEventListError {
    Status401(crate::models::DdosProtectionError),
    Status404(crate::models::DdosProtectionError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ddos_protection_event_rule_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DdosProtectionEventRuleListError {
    Status401(crate::models::DdosProtectionError),
    Status404(crate::models::DdosProtectionError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ddos_protection_rule_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DdosProtectionRuleGetError {
    Status401(crate::models::DdosProtectionError),
    Status404(crate::models::DdosProtectionError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ddos_protection_rule_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DdosProtectionRulePatchError {
    Status400(crate::models::DdosProtectionError),
    Status401(crate::models::DdosProtectionError),
    Status403(crate::models::DdosProtectionError),
    Status404(crate::models::DdosProtectionError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ddos_protection_traffic_stats_rule_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DdosProtectionTrafficStatsRuleGetError {
    Status401(crate::models::DdosProtectionError),
    Status404(crate::models::DdosProtectionError),
    UnknownValue(serde_json::Value),
}


/// Get event by ID.
pub async fn ddos_protection_event_get(configuration: &mut configuration::Configuration, params: DdosProtectionEventGetParams) -> Result<crate::models::DdosProtectionEvent, Error<DdosProtectionEventGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let event_id = params.event_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ddos-protection/v1/events/{event_id}", local_var_configuration.base_path, event_id=crate::apis::urlencode(event_id));
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
        let local_var_entity: Option<DdosProtectionEventGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get events.
pub async fn ddos_protection_event_list(configuration: &mut configuration::Configuration, params: DdosProtectionEventListParams) -> Result<crate::models::InlineResponse2002, Error<DdosProtectionEventListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let cursor = params.cursor;
    let limit = params.limit;
    let service_id = params.service_id;
    let from = params.from;
    let to = params.to;
    let name = params.name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ddos-protection/v1/events", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = service_id {
        local_var_req_builder = local_var_req_builder.query(&[("service_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
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
        let local_var_entity: Option<DdosProtectionEventListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all rules for an event.
pub async fn ddos_protection_event_rule_list(configuration: &mut configuration::Configuration, params: DdosProtectionEventRuleListParams) -> Result<crate::models::InlineResponse2003, Error<DdosProtectionEventRuleListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let event_id = params.event_id;
    let cursor = params.cursor;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ddos-protection/v1/events/{event_id}/rules", local_var_configuration.base_path, event_id=crate::apis::urlencode(event_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DdosProtectionEventRuleListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a rule by ID.
pub async fn ddos_protection_rule_get(configuration: &mut configuration::Configuration, params: DdosProtectionRuleGetParams) -> Result<crate::models::DdosProtectionRule, Error<DdosProtectionRuleGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let rule_id = params.rule_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ddos-protection/v1/rules/{rule_id}", local_var_configuration.base_path, rule_id=crate::apis::urlencode(rule_id));
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
        let local_var_entity: Option<DdosProtectionRuleGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update rule.
pub async fn ddos_protection_rule_patch(configuration: &mut configuration::Configuration, params: DdosProtectionRulePatchParams) -> Result<crate::models::DdosProtectionRule, Error<DdosProtectionRulePatchError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let rule_id = params.rule_id;
    let ddos_protection_rule_patch = params.ddos_protection_rule_patch;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ddos-protection/v1/rules/{rule_id}", local_var_configuration.base_path, rule_id=crate::apis::urlencode(rule_id));
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
    local_var_req_builder = local_var_req_builder.json(&ddos_protection_rule_patch);

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
        let local_var_entity: Option<DdosProtectionRulePatchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get traffic stats for a rule.
pub async fn ddos_protection_traffic_stats_rule_get(configuration: &mut configuration::Configuration, params: DdosProtectionTrafficStatsRuleGetParams) -> Result<crate::models::DdosProtectionTrafficStats, Error<DdosProtectionTrafficStatsRuleGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let event_id = params.event_id;
    let rule_id = params.rule_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ddos-protection/v1/events/{event_id}/rules/{rule_id}/traffic-stats", local_var_configuration.base_path, event_id=crate::apis::urlencode(event_id), rule_id=crate::apis::urlencode(rule_id));
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
        let local_var_entity: Option<DdosProtectionTrafficStatsRuleGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

