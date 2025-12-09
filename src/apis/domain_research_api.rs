/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`domain_status`]
#[derive(Clone, Debug, Default)]
pub struct DomainStatusParams {
    pub domain: String,
    pub scope: Option<String>
}

/// struct for passing parameters to the method [`suggest_domains`]
#[derive(Clone, Debug, Default)]
pub struct SuggestDomainsParams {
    pub query: String,
    pub defaults: Option<String>,
    pub keywords: Option<String>,
    pub location: Option<String>,
    pub vendor: Option<String>
}


/// struct for typed errors of method [`domain_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DomainStatusError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`suggest_domains`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SuggestDomainsError {
    UnknownValue(serde_json::Value),
}


/// The `Status` method checks the availability status of a single domain name.
pub async fn domain_status(configuration: &mut configuration::Configuration, params: DomainStatusParams) -> Result<crate::models::Status, Error<DomainStatusError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let domain = params.domain;
    let scope = params.scope;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/tools/status", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("domain", &domain.to_string())]);
    if let Some(ref local_var_str) = scope {
        local_var_req_builder = local_var_req_builder.query(&[("scope", &local_var_str.to_string())]);
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
        let local_var_entity: Option<DomainStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The `Suggest` method performs a real-time query of the search term(s) against the [known zone database](http://zonedb.org), making recommendations, stemming, and applying Unicode folding, IDN normalization, registrar supported-zone restrictions, and other refinements. **Note:** `Suggest` method responses do not include domain availability status. 
pub async fn suggest_domains(configuration: &mut configuration::Configuration, params: SuggestDomainsParams) -> Result<crate::models::InlineResponse2006, Error<SuggestDomainsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let query = params.query;
    let defaults = params.defaults;
    let keywords = params.keywords;
    let location = params.location;
    let vendor = params.vendor;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/tools/suggest", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("query", &query.to_string())]);
    if let Some(ref local_var_str) = defaults {
        local_var_req_builder = local_var_req_builder.query(&[("defaults", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = keywords {
        local_var_req_builder = local_var_req_builder.query(&[("keywords", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = location {
        local_var_req_builder = local_var_req_builder.query(&[("location", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = vendor {
        local_var_req_builder = local_var_req_builder.query(&[("vendor", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SuggestDomainsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

