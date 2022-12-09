/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_waf_rule_revision`]
#[derive(Clone, Debug, Default)]
pub struct GetWafRuleRevisionParams {
    /// Alphanumeric string identifying a WAF rule.
    pub waf_rule_id: String,
    /// Revision number.
    pub waf_rule_revision_number: i32,
    /// Include relationships. Optional, comma-separated values. Permitted values: `waf_rule`, `vcl`, and `source`. The `vcl` and `source` relationships show the WAF VCL and corresponding ModSecurity source. These fields are blank unless the relationship is included. 
    pub include: Option<String>
}

/// struct for passing parameters to the method [`list_waf_rule_revisions`]
#[derive(Clone, Debug, Default)]
pub struct ListWafRuleRevisionsParams {
    /// Alphanumeric string identifying a WAF rule.
    pub waf_rule_id: String,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>,
    /// Include relationships. Optional.
    pub include: Option<String>
}


/// struct for typed errors of method [`get_waf_rule_revision`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWafRuleRevisionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_waf_rule_revisions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWafRuleRevisionsError {
    UnknownValue(serde_json::Value),
}


/// Get a specific rule revision.
pub async fn get_waf_rule_revision(configuration: &mut configuration::Configuration, params: GetWafRuleRevisionParams) -> Result<crate::models::WafRuleRevisionResponse, Error<GetWafRuleRevisionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let waf_rule_id = params.waf_rule_id;
    let waf_rule_revision_number = params.waf_rule_revision_number;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/rules/{waf_rule_id}/revisions/{waf_rule_revision_number}", local_var_configuration.base_path, waf_rule_id=crate::apis::urlencode(waf_rule_id), waf_rule_revision_number=waf_rule_revision_number);
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
        let local_var_entity: Option<GetWafRuleRevisionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all revisions for a specific rule. The `rule_id` provided can be the ModSecurity Rule ID or the Fastly generated rule ID.
pub async fn list_waf_rule_revisions(configuration: &mut configuration::Configuration, params: ListWafRuleRevisionsParams) -> Result<crate::models::WafRuleRevisionsResponse, Error<ListWafRuleRevisionsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let waf_rule_id = params.waf_rule_id;
    let page_number = params.page_number;
    let page_size = params.page_size;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/waf/rules/{waf_rule_id}/revisions", local_var_configuration.base_path, waf_rule_id=crate::apis::urlencode(waf_rule_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<ListWafRuleRevisionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

