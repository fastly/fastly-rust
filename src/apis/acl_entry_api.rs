/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`bulk_update_acl_entries`]
#[derive(Clone, Debug, Default)]
pub struct BulkUpdateAclEntriesParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a ACL.
    pub acl_id: String,
    pub bulk_update_acl_entries_request: Option<crate::models::BulkUpdateAclEntriesRequest>
}

/// struct for passing parameters to the method [`create_acl_entry`]
#[derive(Clone, Debug, Default)]
pub struct CreateAclEntryParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a ACL.
    pub acl_id: String,
    pub acl_entry: Option<crate::models::AclEntry>
}

/// struct for passing parameters to the method [`delete_acl_entry`]
#[derive(Clone, Debug, Default)]
pub struct DeleteAclEntryParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a ACL.
    pub acl_id: String,
    /// Alphanumeric string identifying an ACL Entry.
    pub acl_entry_id: String
}

/// struct for passing parameters to the method [`get_acl_entry`]
#[derive(Clone, Debug, Default)]
pub struct GetAclEntryParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a ACL.
    pub acl_id: String,
    /// Alphanumeric string identifying an ACL Entry.
    pub acl_entry_id: String
}

/// struct for passing parameters to the method [`list_acl_entries`]
#[derive(Clone, Debug, Default)]
pub struct ListAclEntriesParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a ACL.
    pub acl_id: String,
    /// Current page.
    pub page: Option<i32>,
    /// Number of records per page.
    pub per_page: Option<i32>,
    /// Field on which to sort.
    pub sort: Option<String>,
    /// Direction in which to sort results.
    pub direction: Option<String>
}

/// struct for passing parameters to the method [`update_acl_entry`]
#[derive(Clone, Debug, Default)]
pub struct UpdateAclEntryParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Alphanumeric string identifying a ACL.
    pub acl_id: String,
    /// Alphanumeric string identifying an ACL Entry.
    pub acl_entry_id: String,
    pub acl_entry: Option<crate::models::AclEntry>
}


/// struct for typed errors of method [`bulk_update_acl_entries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkUpdateAclEntriesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_acl_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAclEntryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_acl_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAclEntryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_acl_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAclEntryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_acl_entries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAclEntriesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_acl_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAclEntryError {
    UnknownValue(serde_json::Value),
}


/// Update multiple ACL entries on the same ACL. For faster updates to your service, group your changes into large batches. The maximum batch size is 1000 entries. [Contact support](https://support.fastly.com/) to discuss raising this limit.
pub async fn bulk_update_acl_entries(configuration: &mut configuration::Configuration, params: BulkUpdateAclEntriesParams) -> Result<crate::models::InlineResponse200, Error<BulkUpdateAclEntriesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let acl_id = params.acl_id;
    let bulk_update_acl_entries_request = params.bulk_update_acl_entries_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/acl/{acl_id}/entries", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), acl_id=crate::apis::urlencode(acl_id));
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
    local_var_req_builder = local_var_req_builder.json(&bulk_update_acl_entries_request);

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
        let local_var_entity: Option<BulkUpdateAclEntriesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Add an ACL entry to an ACL.
pub async fn create_acl_entry(configuration: &mut configuration::Configuration, params: CreateAclEntryParams) -> Result<crate::models::AclEntryResponse, Error<CreateAclEntryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let acl_id = params.acl_id;
    let acl_entry = params.acl_entry;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/acl/{acl_id}/entry", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), acl_id=crate::apis::urlencode(acl_id));
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
    local_var_req_builder = local_var_req_builder.json(&acl_entry);

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
        let local_var_entity: Option<CreateAclEntryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an ACL entry from a specified ACL.
pub async fn delete_acl_entry(configuration: &mut configuration::Configuration, params: DeleteAclEntryParams) -> Result<crate::models::InlineResponse200, Error<DeleteAclEntryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let acl_id = params.acl_id;
    let acl_entry_id = params.acl_entry_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/acl/{acl_id}/entry/{acl_entry_id}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), acl_id=crate::apis::urlencode(acl_id), acl_entry_id=crate::apis::urlencode(acl_entry_id));
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteAclEntryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a single ACL entry.
pub async fn get_acl_entry(configuration: &mut configuration::Configuration, params: GetAclEntryParams) -> Result<crate::models::AclEntryResponse, Error<GetAclEntryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let acl_id = params.acl_id;
    let acl_entry_id = params.acl_entry_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/acl/{acl_id}/entry/{acl_entry_id}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), acl_id=crate::apis::urlencode(acl_id), acl_entry_id=crate::apis::urlencode(acl_entry_id));
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
        let local_var_entity: Option<GetAclEntryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List ACL entries for a specified ACL.
pub async fn list_acl_entries(configuration: &mut configuration::Configuration, params: ListAclEntriesParams) -> Result<Vec<crate::models::AclEntryResponse>, Error<ListAclEntriesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let acl_id = params.acl_id;
    let page = params.page;
    let per_page = params.per_page;
    let sort = params.sort;
    let direction = params.direction;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/acl/{acl_id}/entries", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), acl_id=crate::apis::urlencode(acl_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = direction {
        local_var_req_builder = local_var_req_builder.query(&[("direction", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListAclEntriesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update an ACL entry for a specified ACL.
pub async fn update_acl_entry(configuration: &mut configuration::Configuration, params: UpdateAclEntryParams) -> Result<crate::models::AclEntryResponse, Error<UpdateAclEntryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let acl_id = params.acl_id;
    let acl_entry_id = params.acl_entry_id;
    let acl_entry = params.acl_entry;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/acl/{acl_id}/entry/{acl_entry_id}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), acl_id=crate::apis::urlencode(acl_id), acl_entry_id=crate::apis::urlencode(acl_entry_id));
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
    local_var_req_builder = local_var_req_builder.json(&acl_entry);

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
        let local_var_entity: Option<UpdateAclEntryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

