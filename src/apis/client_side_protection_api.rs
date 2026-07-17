/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`csp_create_page`]
#[derive(Clone, Debug, Default)]
pub struct CspCreatePageParams {
    pub page_create: Option<crate::models::PageCreate>
}

/// struct for passing parameters to the method [`csp_create_policy`]
#[derive(Clone, Debug, Default)]
pub struct CspCreatePolicyParams {
    /// Page identifier
    pub page_id: String,
    pub policy_create: Option<crate::models::PolicyCreate>
}

/// struct for passing parameters to the method [`csp_create_website`]
#[derive(Clone, Debug, Default)]
pub struct CspCreateWebsiteParams {
    pub website_create: Option<crate::models::WebsiteCreate>
}

/// struct for passing parameters to the method [`csp_delete_page`]
#[derive(Clone, Debug, Default)]
pub struct CspDeletePageParams {
    /// Page identifier
    pub page_id: String
}

/// struct for passing parameters to the method [`csp_delete_website`]
#[derive(Clone, Debug, Default)]
pub struct CspDeleteWebsiteParams {
    /// Website identifier
    pub website_id: String
}

/// struct for passing parameters to the method [`csp_get_page`]
#[derive(Clone, Debug, Default)]
pub struct CspGetPageParams {
    /// Page identifier
    pub page_id: String
}

/// struct for passing parameters to the method [`csp_get_policy`]
#[derive(Clone, Debug, Default)]
pub struct CspGetPolicyParams {
    /// Page identifier
    pub page_id: String,
    /// Policy identifier
    pub policy_id: String
}

/// struct for passing parameters to the method [`csp_get_script`]
#[derive(Clone, Debug, Default)]
pub struct CspGetScriptParams {
    /// Page identifier
    pub page_id: String,
    /// Script identifier
    pub script_id: String
}

/// struct for passing parameters to the method [`csp_get_website`]
#[derive(Clone, Debug, Default)]
pub struct CspGetWebsiteParams {
    /// Website identifier
    pub website_id: String
}

/// struct for passing parameters to the method [`csp_list_header_events`]
#[derive(Clone, Debug, Default)]
pub struct CspListHeaderEventsParams {
    /// Page identifier
    pub page_id: String,
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Page number of the collection to request.
    pub page: Option<i32>
}

/// struct for passing parameters to the method [`csp_list_headers`]
#[derive(Clone, Debug, Default)]
pub struct CspListHeadersParams {
    /// Page identifier
    pub page_id: String,
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Page number of the collection to request.
    pub page: Option<i32>
}

/// struct for passing parameters to the method [`csp_list_pages`]
#[derive(Clone, Debug, Default)]
pub struct CspListPagesParams {
    /// Filter pages by website ID
    pub website_id: Option<String>,
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Page number of the collection to request.
    pub page: Option<i32>
}

/// struct for passing parameters to the method [`csp_list_policies`]
#[derive(Clone, Debug, Default)]
pub struct CspListPoliciesParams {
    /// Page identifier
    pub page_id: String,
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Page number of the collection to request.
    pub page: Option<i32>
}

/// struct for passing parameters to the method [`csp_list_policy_reports`]
#[derive(Clone, Debug, Default)]
pub struct CspListPolicyReportsParams {
    /// Page identifier
    pub page_id: String,
    /// Policy identifier
    pub policy_id: String,
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Page number of the collection to request.
    pub page: Option<i32>
}

/// struct for passing parameters to the method [`csp_list_scripts`]
#[derive(Clone, Debug, Default)]
pub struct CspListScriptsParams {
    /// Page identifier
    pub page_id: String,
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Page number of the collection to request.
    pub page: Option<i32>
}

/// struct for passing parameters to the method [`csp_list_websites`]
#[derive(Clone, Debug, Default)]
pub struct CspListWebsitesParams {
    /// Limit how many results are returned.
    pub limit: Option<i32>,
    /// Page number of the collection to request.
    pub page: Option<i32>
}

/// struct for passing parameters to the method [`csp_update_page`]
#[derive(Clone, Debug, Default)]
pub struct CspUpdatePageParams {
    /// Page identifier
    pub page_id: String,
    pub page_update: Option<crate::models::PageUpdate>
}

/// struct for passing parameters to the method [`csp_update_policy`]
#[derive(Clone, Debug, Default)]
pub struct CspUpdatePolicyParams {
    /// Page identifier
    pub page_id: String,
    /// Policy identifier
    pub policy_id: String,
    pub policy_update: Option<crate::models::PolicyUpdate>
}

/// struct for passing parameters to the method [`csp_update_script`]
#[derive(Clone, Debug, Default)]
pub struct CspUpdateScriptParams {
    /// Page identifier
    pub page_id: String,
    /// Script identifier
    pub script_id: String,
    pub script_update: Option<crate::models::ScriptUpdate>
}

/// struct for passing parameters to the method [`csp_update_website`]
#[derive(Clone, Debug, Default)]
pub struct CspUpdateWebsiteParams {
    /// Website identifier
    pub website_id: String,
    pub website_update: Option<crate::models::WebsiteUpdate>
}


/// struct for typed errors of method [`csp_create_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspCreatePageError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_create_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspCreatePolicyError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_create_website`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspCreateWebsiteError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_delete_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspDeletePageError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_delete_website`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspDeleteWebsiteError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_get_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspGetPageError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_get_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspGetPolicyError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_get_script`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspGetScriptError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_get_website`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspGetWebsiteError {
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_list_header_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspListHeaderEventsError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_list_headers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspListHeadersError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_list_pages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspListPagesError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_list_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspListPoliciesError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_list_policy_reports`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspListPolicyReportsError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_list_scripts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspListScriptsError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_list_websites`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspListWebsitesError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_update_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspUpdatePageError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_update_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspUpdatePolicyError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_update_script`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspUpdateScriptError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`csp_update_website`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CspUpdateWebsiteError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status403(serde_json::Value),
    Status404(serde_json::Value),
    Status429(serde_json::Value),
    UnknownValue(serde_json::Value),
}


/// Create a new page for monitoring.
pub async fn csp_create_page(configuration: &mut configuration::Configuration, params: CspCreatePageParams) -> Result<crate::models::Page, Error<CspCreatePageError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_create = params.page_create;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&page_create);

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
        let local_var_entity: Option<CspCreatePageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new Content Security Policy for a page.
pub async fn csp_create_policy(configuration: &mut configuration::Configuration, params: CspCreatePolicyParams) -> Result<crate::models::Policy, Error<CspCreatePolicyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;
    let policy_create = params.policy_create;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}/policies", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id));
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
    local_var_req_builder = local_var_req_builder.json(&policy_create);

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
        let local_var_entity: Option<CspCreatePolicyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new website for Client-Side Protection monitoring.
pub async fn csp_create_website(configuration: &mut configuration::Configuration, params: CspCreateWebsiteParams) -> Result<crate::models::Website, Error<CspCreateWebsiteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let website_create = params.website_create;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/websites", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&website_create);

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
        let local_var_entity: Option<CspCreateWebsiteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a page and all associated scripts and policies.
pub async fn csp_delete_page(configuration: &mut configuration::Configuration, params: CspDeletePageParams) -> Result<(), Error<CspDeletePageError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id));
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
        let local_var_entity: Option<CspDeletePageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a website and all associated pages, scripts, and policies.
pub async fn csp_delete_website(configuration: &mut configuration::Configuration, params: CspDeleteWebsiteParams) -> Result<(), Error<CspDeleteWebsiteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let website_id = params.website_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/websites/{website_id}", local_var_configuration.base_path, website_id=crate::apis::urlencode(website_id));
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
        let local_var_entity: Option<CspDeleteWebsiteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get details for a specific page.
pub async fn csp_get_page(configuration: &mut configuration::Configuration, params: CspGetPageParams) -> Result<crate::models::Page, Error<CspGetPageError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id));
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
        let local_var_entity: Option<CspGetPageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get details for a specific policy.
pub async fn csp_get_policy(configuration: &mut configuration::Configuration, params: CspGetPolicyParams) -> Result<crate::models::Policy, Error<CspGetPolicyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;
    let policy_id = params.policy_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}/policies/{policy_id}", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id), policy_id=crate::apis::urlencode(policy_id));
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
        let local_var_entity: Option<CspGetPolicyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get details for a specific script.
pub async fn csp_get_script(configuration: &mut configuration::Configuration, params: CspGetScriptParams) -> Result<crate::models::Script, Error<CspGetScriptError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;
    let script_id = params.script_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}/scripts/{script_id}", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id), script_id=crate::apis::urlencode(script_id));
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
        let local_var_entity: Option<CspGetScriptError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get details for a specific website.
pub async fn csp_get_website(configuration: &mut configuration::Configuration, params: CspGetWebsiteParams) -> Result<crate::models::Website, Error<CspGetWebsiteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let website_id = params.website_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/websites/{website_id}", local_var_configuration.base_path, website_id=crate::apis::urlencode(website_id));
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
        let local_var_entity: Option<CspGetWebsiteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List security header change events for a page.
pub async fn csp_list_header_events(configuration: &mut configuration::Configuration, params: CspListHeaderEventsParams) -> Result<crate::models::InlineResponse20011, Error<CspListHeaderEventsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;
    let limit = params.limit;
    let page = params.page;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}/events", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
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
        let local_var_entity: Option<CspListHeaderEventsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List security headers detected on a page.
pub async fn csp_list_headers(configuration: &mut configuration::Configuration, params: CspListHeadersParams) -> Result<crate::models::InlineResponse20010, Error<CspListHeadersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;
    let limit = params.limit;
    let page = params.page;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}/headers", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
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
        let local_var_entity: Option<CspListHeadersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all pages. Optionally filter by website.
pub async fn csp_list_pages(configuration: &mut configuration::Configuration, params: CspListPagesParams) -> Result<crate::models::InlineResponse2006, Error<CspListPagesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let website_id = params.website_id;
    let limit = params.limit;
    let page = params.page;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = website_id {
        local_var_req_builder = local_var_req_builder.query(&[("website_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
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
        let local_var_entity: Option<CspListPagesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all Content Security Policies for a page.
pub async fn csp_list_policies(configuration: &mut configuration::Configuration, params: CspListPoliciesParams) -> Result<crate::models::InlineResponse2008, Error<CspListPoliciesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;
    let limit = params.limit;
    let page = params.page;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}/policies", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
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
        let local_var_entity: Option<CspListPoliciesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List CSP violation reports for a policy.
pub async fn csp_list_policy_reports(configuration: &mut configuration::Configuration, params: CspListPolicyReportsParams) -> Result<crate::models::InlineResponse2009, Error<CspListPolicyReportsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;
    let policy_id = params.policy_id;
    let limit = params.limit;
    let page = params.page;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}/policies/{policy_id}/reports", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id), policy_id=crate::apis::urlencode(policy_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
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
        let local_var_entity: Option<CspListPolicyReportsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all scripts detected on a page.
pub async fn csp_list_scripts(configuration: &mut configuration::Configuration, params: CspListScriptsParams) -> Result<crate::models::InlineResponse2007, Error<CspListScriptsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;
    let limit = params.limit;
    let page = params.page;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}/scripts", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
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
        let local_var_entity: Option<CspListScriptsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all websites configured for Client-Side Protection.
pub async fn csp_list_websites(configuration: &mut configuration::Configuration, params: CspListWebsitesParams) -> Result<crate::models::InlineResponse2005, Error<CspListWebsitesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let limit = params.limit;
    let page = params.page;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/websites", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
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
        let local_var_entity: Option<CspListWebsitesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a page's configuration.
pub async fn csp_update_page(configuration: &mut configuration::Configuration, params: CspUpdatePageParams) -> Result<crate::models::Page, Error<CspUpdatePageError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;
    let page_update = params.page_update;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id));
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
    local_var_req_builder = local_var_req_builder.json(&page_update);

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
        let local_var_entity: Option<CspUpdatePageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a policy's configuration.
pub async fn csp_update_policy(configuration: &mut configuration::Configuration, params: CspUpdatePolicyParams) -> Result<crate::models::Policy, Error<CspUpdatePolicyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;
    let policy_id = params.policy_id;
    let policy_update = params.policy_update;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}/policies/{policy_id}", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id), policy_id=crate::apis::urlencode(policy_id));
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
    local_var_req_builder = local_var_req_builder.json(&policy_update);

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
        let local_var_entity: Option<CspUpdatePolicyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a script's authorization status or justification.
pub async fn csp_update_script(configuration: &mut configuration::Configuration, params: CspUpdateScriptParams) -> Result<crate::models::Script, Error<CspUpdateScriptError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_id = params.page_id;
    let script_id = params.script_id;
    let script_update = params.script_update;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/pages/{page_id}/scripts/{script_id}", local_var_configuration.base_path, page_id=crate::apis::urlencode(page_id), script_id=crate::apis::urlencode(script_id));
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
    local_var_req_builder = local_var_req_builder.json(&script_update);

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
        let local_var_entity: Option<CspUpdateScriptError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a website's configuration.
pub async fn csp_update_website(configuration: &mut configuration::Configuration, params: CspUpdateWebsiteParams) -> Result<crate::models::Website, Error<CspUpdateWebsiteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let website_id = params.website_id;
    let website_update = params.website_update;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/client-side-protection/v1/websites/{website_id}", local_var_configuration.base_path, website_id=crate::apis::urlencode(website_id));
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
    local_var_req_builder = local_var_req_builder.json(&website_update);

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
        let local_var_entity: Option<CspUpdateWebsiteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

