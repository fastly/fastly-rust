/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_dm_domain`]
#[derive(Clone, Debug, Default)]
pub struct CreateDmDomainParams {
    pub request_body_for_create: Option<crate::models::RequestBodyForCreate>
}

/// struct for passing parameters to the method [`delete_dm_domain`]
#[derive(Clone, Debug, Default)]
pub struct DeleteDmDomainParams {
    pub domain_id: String
}

/// struct for passing parameters to the method [`get_dm_domain`]
#[derive(Clone, Debug, Default)]
pub struct GetDmDomainParams {
    pub domain_id: String
}

/// struct for passing parameters to the method [`list_dm_domains`]
#[derive(Clone, Debug, Default)]
pub struct ListDmDomainsParams {
    pub fqdn: Option<String>,
    /// (Optional) Filter fully-qualified domain name (FQDN) specifically by match type. If used, requires filtering by FQDN.
    pub fqdn_match: Option<String>,
    /// Filter results based on a service_id.
    pub service_id: Option<String>,
    /// The order in which to list the results.
    pub sort: Option<String>,
    pub activated: Option<bool>,
    pub verified: Option<bool>,
    /// Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty.
    pub cursor: Option<String>,
    /// Limit how many results are returned.
    pub limit: Option<i32>
}

/// struct for passing parameters to the method [`update_dm_domain`]
#[derive(Clone, Debug, Default)]
pub struct UpdateDmDomainParams {
    pub domain_id: String,
    pub request_body_for_update: Option<crate::models::RequestBodyForUpdate>
}


/// struct for typed errors of method [`create_dm_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDmDomainError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_dm_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDmDomainError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dm_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDmDomainError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_dm_domains`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDmDomainsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_dm_domain`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDmDomainError {
    UnknownValue(serde_json::Value),
}


/// Create a domain
pub async fn create_dm_domain(configuration: &mut configuration::Configuration, params: CreateDmDomainParams) -> Result<crate::models::SuccessfulResponseAsObject, Error<CreateDmDomainError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let request_body_for_create = params.request_body_for_create;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/domains", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&request_body_for_create);

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
        let local_var_entity: Option<CreateDmDomainError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a domain
pub async fn delete_dm_domain(configuration: &mut configuration::Configuration, params: DeleteDmDomainParams) -> Result<(), Error<DeleteDmDomainError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let domain_id = params.domain_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/domains/{domain_id}", local_var_configuration.base_path, domain_id=crate::apis::urlencode(domain_id));
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
        let local_var_entity: Option<DeleteDmDomainError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Show a domain
pub async fn get_dm_domain(configuration: &mut configuration::Configuration, params: GetDmDomainParams) -> Result<crate::models::SuccessfulResponseAsObject, Error<GetDmDomainError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let domain_id = params.domain_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/domains/{domain_id}", local_var_configuration.base_path, domain_id=crate::apis::urlencode(domain_id));
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
        let local_var_entity: Option<GetDmDomainError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all domains
pub async fn list_dm_domains(configuration: &mut configuration::Configuration, params: ListDmDomainsParams) -> Result<crate::models::InlineResponse2007, Error<ListDmDomainsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let fqdn = params.fqdn;
    let fqdn_match = params.fqdn_match;
    let service_id = params.service_id;
    let sort = params.sort;
    let activated = params.activated;
    let verified = params.verified;
    let cursor = params.cursor;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/domains", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fqdn {
        local_var_req_builder = local_var_req_builder.query(&[("fqdn", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fqdn_match {
        local_var_req_builder = local_var_req_builder.query(&[("fqdn_match", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = service_id {
        local_var_req_builder = local_var_req_builder.query(&[("service_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = activated {
        local_var_req_builder = local_var_req_builder.query(&[("activated", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = verified {
        local_var_req_builder = local_var_req_builder.query(&[("verified", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<ListDmDomainsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a domain
pub async fn update_dm_domain(configuration: &mut configuration::Configuration, params: UpdateDmDomainParams) -> Result<crate::models::SuccessfulResponseAsObject, Error<UpdateDmDomainError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let domain_id = params.domain_id;
    let request_body_for_update = params.request_body_for_update;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/domain-management/v1/domains/{domain_id}", local_var_configuration.base_path, domain_id=crate::apis::urlencode(domain_id));
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
    local_var_req_builder = local_var_req_builder.json(&request_body_for_update);

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
        let local_var_entity: Option<UpdateDmDomainError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

