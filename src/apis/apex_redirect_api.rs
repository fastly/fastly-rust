/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_apex_redirect`]
#[derive(Clone, Debug, Default)]
pub struct CreateApexRedirectParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    pub service_id2: Option<String>,
    pub version: Option<i32>,
    /// Date and time in ISO 8601 format.
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    pub updated_at: Option<String>,
    /// HTTP status code used to redirect the client.
    pub status_code: Option<i32>,
    /// Array of apex domains that should redirect to their WWW subdomain.
    pub domains: Option<Vec<String>>,
    /// Revision number of the apex redirect feature implementation. Defaults to the most recent revision.
    pub feature_revision: Option<i32>
}

/// struct for passing parameters to the method [`delete_apex_redirect`]
#[derive(Clone, Debug, Default)]
pub struct DeleteApexRedirectParams {
    pub apex_redirect_id: String
}

/// struct for passing parameters to the method [`get_apex_redirect`]
#[derive(Clone, Debug, Default)]
pub struct GetApexRedirectParams {
    pub apex_redirect_id: String
}

/// struct for passing parameters to the method [`list_apex_redirects`]
#[derive(Clone, Debug, Default)]
pub struct ListApexRedirectsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32
}

/// struct for passing parameters to the method [`update_apex_redirect`]
#[derive(Clone, Debug, Default)]
pub struct UpdateApexRedirectParams {
    pub apex_redirect_id: String,
    pub service_id: Option<String>,
    pub version: Option<i32>,
    /// Date and time in ISO 8601 format.
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    pub updated_at: Option<String>,
    /// HTTP status code used to redirect the client.
    pub status_code: Option<i32>,
    /// Array of apex domains that should redirect to their WWW subdomain.
    pub domains: Option<Vec<String>>,
    /// Revision number of the apex redirect feature implementation. Defaults to the most recent revision.
    pub feature_revision: Option<i32>
}


/// struct for typed errors of method [`create_apex_redirect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApexRedirectError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_apex_redirect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApexRedirectError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_apex_redirect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApexRedirectError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_apex_redirects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApexRedirectsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_apex_redirect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApexRedirectError {
    UnknownValue(serde_json::Value),
}


/// Create an apex redirect for a particular service and version.
pub async fn create_apex_redirect(configuration: &mut configuration::Configuration, params: CreateApexRedirectParams) -> Result<crate::models::ApexRedirect, Error<CreateApexRedirectError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let service_id2 = params.service_id2;
    let version = params.version;
    let created_at = params.created_at;
    let deleted_at = params.deleted_at;
    let updated_at = params.updated_at;
    let status_code = params.status_code;
    let domains = params.domains;
    let feature_revision = params.feature_revision;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/apex-redirects", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = service_id2 {
        local_var_form_params.insert("service_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = version {
        local_var_form_params.insert("version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = created_at {
        local_var_form_params.insert("created_at", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = deleted_at {
        local_var_form_params.insert("deleted_at", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = updated_at {
        local_var_form_params.insert("updated_at", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_code {
        local_var_form_params.insert("status_code", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = domains {
        local_var_form_params.insert("domains", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    if let Some(local_var_param_value) = feature_revision {
        local_var_form_params.insert("feature_revision", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

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
        let local_var_entity: Option<CreateApexRedirectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an apex redirect by its ID.
pub async fn delete_apex_redirect(configuration: &mut configuration::Configuration, params: DeleteApexRedirectParams) -> Result<crate::models::InlineResponse200, Error<DeleteApexRedirectError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let apex_redirect_id = params.apex_redirect_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/apex-redirects/{apex_redirect_id}", local_var_configuration.base_path, apex_redirect_id=crate::apis::urlencode(apex_redirect_id));
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
        let local_var_entity: Option<DeleteApexRedirectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get an apex redirect by its ID.
pub async fn get_apex_redirect(configuration: &mut configuration::Configuration, params: GetApexRedirectParams) -> Result<crate::models::ApexRedirect, Error<GetApexRedirectError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let apex_redirect_id = params.apex_redirect_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/apex-redirects/{apex_redirect_id}", local_var_configuration.base_path, apex_redirect_id=crate::apis::urlencode(apex_redirect_id));
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
        let local_var_entity: Option<GetApexRedirectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all apex redirects for a particular service and version.
pub async fn list_apex_redirects(configuration: &mut configuration::Configuration, params: ListApexRedirectsParams) -> Result<Vec<crate::models::ApexRedirect>, Error<ListApexRedirectsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/apex-redirects", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
        let local_var_entity: Option<ListApexRedirectsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update an apex redirect by its ID.
pub async fn update_apex_redirect(configuration: &mut configuration::Configuration, params: UpdateApexRedirectParams) -> Result<crate::models::ApexRedirect, Error<UpdateApexRedirectError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let apex_redirect_id = params.apex_redirect_id;
    let service_id = params.service_id;
    let version = params.version;
    let created_at = params.created_at;
    let deleted_at = params.deleted_at;
    let updated_at = params.updated_at;
    let status_code = params.status_code;
    let domains = params.domains;
    let feature_revision = params.feature_revision;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/apex-redirects/{apex_redirect_id}", local_var_configuration.base_path, apex_redirect_id=crate::apis::urlencode(apex_redirect_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = service_id {
        local_var_form_params.insert("service_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = version {
        local_var_form_params.insert("version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = created_at {
        local_var_form_params.insert("created_at", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = deleted_at {
        local_var_form_params.insert("deleted_at", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = updated_at {
        local_var_form_params.insert("updated_at", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = status_code {
        local_var_form_params.insert("status_code", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = domains {
        local_var_form_params.insert("domains", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    if let Some(local_var_param_value) = feature_revision {
        local_var_form_params.insert("feature_revision", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "PUT" != "GET" && "PUT" != "HEAD" {
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
        let local_var_entity: Option<UpdateApexRedirectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

