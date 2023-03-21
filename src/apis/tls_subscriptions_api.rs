/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_globalsign_email_challenge`]
#[derive(Clone, Debug, Default)]
pub struct CreateGlobalsignEmailChallengeParams {
    /// Alphanumeric string identifying a TLS subscription.
    pub tls_subscription_id: String,
    /// Alphanumeric string identifying a TLS subscription.
    pub tls_authorization_id: String,
    pub request_body: Option<::std::collections::HashMap<String, serde_json::Value>>
}

/// struct for passing parameters to the method [`create_tls_sub`]
#[derive(Clone, Debug, Default)]
pub struct CreateTlsSubParams {
    /// A flag that allows you to edit and delete a subscription with active domains. Valid to use on PATCH and DELETE actions. As a warning, removing an active domain from a subscription or forcing the deletion of a subscription may result in breaking TLS termination to that domain. 
    pub force: Option<bool>,
    pub tls_subscription: Option<crate::models::TlsSubscription>
}

/// struct for passing parameters to the method [`delete_globalsign_email_challenge`]
#[derive(Clone, Debug, Default)]
pub struct DeleteGlobalsignEmailChallengeParams {
    /// Alphanumeric string identifying a TLS subscription.
    pub tls_subscription_id: String,
    /// Alphanumeric string identifying a GlobalSign email challenge.
    pub globalsign_email_challenge_id: String,
    /// Alphanumeric string identifying a TLS subscription.
    pub tls_authorization_id: String
}

/// struct for passing parameters to the method [`delete_tls_sub`]
#[derive(Clone, Debug, Default)]
pub struct DeleteTlsSubParams {
    /// Alphanumeric string identifying a TLS subscription.
    pub tls_subscription_id: String
}

/// struct for passing parameters to the method [`get_tls_sub`]
#[derive(Clone, Debug, Default)]
pub struct GetTlsSubParams {
    /// Alphanumeric string identifying a TLS subscription.
    pub tls_subscription_id: String,
    /// Include related objects. Optional, comma-separated values. Permitted values: `tls_authorizations` and `tls_authorizations.globalsign_email_challenge`. 
    pub include: Option<String>
}

/// struct for passing parameters to the method [`list_tls_subs`]
#[derive(Clone, Debug, Default)]
pub struct ListTlsSubsParams {
    /// Limit the returned subscriptions by state. Valid values are `pending`, `processing`, `issued`, `renewing`, and `failed`. Accepts parameters: `not` (e.g., `filter[state][not]=renewing`). 
    pub filter_state: Option<String>,
    /// Limit the returned subscriptions to those that include the specific domain.
    pub filter_tls_domains_id: Option<String>,
    /// Limit the returned subscriptions to those that have currently active orders. Permitted values: `true`. 
    pub filter_has_active_order: Option<bool>,
    /// Include related objects. Optional, comma-separated values. Permitted values: `tls_authorizations` and `tls_authorizations.globalsign_email_challenge`. 
    pub include: Option<String>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>,
    /// The order in which to list the results by creation date.
    pub sort: Option<String>
}

/// struct for passing parameters to the method [`patch_tls_sub`]
#[derive(Clone, Debug, Default)]
pub struct PatchTlsSubParams {
    /// Alphanumeric string identifying a TLS subscription.
    pub tls_subscription_id: String,
    /// A flag that allows you to edit and delete a subscription with active domains. Valid to use on PATCH and DELETE actions. As a warning, removing an active domain from a subscription or forcing the deletion of a subscription may result in breaking TLS termination to that domain. 
    pub force: Option<bool>,
    pub tls_subscription: Option<crate::models::TlsSubscription>
}


/// struct for typed errors of method [`create_globalsign_email_challenge`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGlobalsignEmailChallengeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_tls_sub`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTlsSubError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_globalsign_email_challenge`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGlobalsignEmailChallengeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_tls_sub`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTlsSubError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tls_sub`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTlsSubError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_tls_subs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTlsSubsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`patch_tls_sub`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchTlsSubError {
    UnknownValue(serde_json::Value),
}


/// Creates an email challenge for a domain on a GlobalSign subscription. An email challenge will generate an email that can be used to validate domain ownership. If this challenge is created, then the domain can only be validated using email for the given subscription. 
pub async fn create_globalsign_email_challenge(configuration: &mut configuration::Configuration, params: CreateGlobalsignEmailChallengeParams) -> Result<serde_json::Value, Error<CreateGlobalsignEmailChallengeError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_subscription_id = params.tls_subscription_id;
    let tls_authorization_id = params.tls_authorization_id;
    let request_body = params.request_body;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/subscriptions/{tls_subscription_id}/authorizations/{tls_authorization_id}/globalsign_email_challenges", local_var_configuration.base_path, tls_subscription_id=crate::apis::urlencode(tls_subscription_id), tls_authorization_id=crate::apis::urlencode(tls_authorization_id));
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
        let local_var_entity: Option<CreateGlobalsignEmailChallengeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new TLS subscription. This response includes a list of possible challenges to verify domain ownership.
pub async fn create_tls_sub(configuration: &mut configuration::Configuration, params: CreateTlsSubParams) -> Result<crate::models::TlsSubscriptionResponse, Error<CreateTlsSubError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let force = params.force;
    let tls_subscription = params.tls_subscription;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/subscriptions", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&tls_subscription);

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
        let local_var_entity: Option<CreateTlsSubError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a GlobalSign email challenge. After a GlobalSign email challenge is deleted, the domain can use HTTP and DNS validation methods again.
pub async fn delete_globalsign_email_challenge(configuration: &mut configuration::Configuration, params: DeleteGlobalsignEmailChallengeParams) -> Result<(), Error<DeleteGlobalsignEmailChallengeError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_subscription_id = params.tls_subscription_id;
    let globalsign_email_challenge_id = params.globalsign_email_challenge_id;
    let tls_authorization_id = params.tls_authorization_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/subscriptions/{tls_subscription_id}/authorizations/{tls_authorization_id}/globalsign_email_challenges/{globalsign_email_challenge_id}", local_var_configuration.base_path, tls_subscription_id=crate::apis::urlencode(tls_subscription_id), globalsign_email_challenge_id=crate::apis::urlencode(globalsign_email_challenge_id), tls_authorization_id=crate::apis::urlencode(tls_authorization_id));
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
        let local_var_entity: Option<DeleteGlobalsignEmailChallengeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Destroy a TLS subscription. A subscription cannot be destroyed if there are domains in the TLS enabled state.
pub async fn delete_tls_sub(configuration: &mut configuration::Configuration, params: DeleteTlsSubParams) -> Result<(), Error<DeleteTlsSubError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_subscription_id = params.tls_subscription_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/subscriptions/{tls_subscription_id}", local_var_configuration.base_path, tls_subscription_id=crate::apis::urlencode(tls_subscription_id));
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
        let local_var_entity: Option<DeleteTlsSubError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Show a TLS subscription.
pub async fn get_tls_sub(configuration: &mut configuration::Configuration, params: GetTlsSubParams) -> Result<crate::models::TlsSubscriptionResponse, Error<GetTlsSubError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_subscription_id = params.tls_subscription_id;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/subscriptions/{tls_subscription_id}", local_var_configuration.base_path, tls_subscription_id=crate::apis::urlencode(tls_subscription_id));
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
        let local_var_entity: Option<GetTlsSubError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all TLS subscriptions.
pub async fn list_tls_subs(configuration: &mut configuration::Configuration, params: ListTlsSubsParams) -> Result<crate::models::TlsSubscriptionsResponse, Error<ListTlsSubsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let filter_state = params.filter_state;
    let filter_tls_domains_id = params.filter_tls_domains_id;
    let filter_has_active_order = params.filter_has_active_order;
    let include = params.include;
    let page_number = params.page_number;
    let page_size = params.page_size;
    let sort = params.sort;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/subscriptions", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_state {
        local_var_req_builder = local_var_req_builder.query(&[("filter[state]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_tls_domains_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[tls_domains.id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_has_active_order {
        local_var_req_builder = local_var_req_builder.query(&[("filter[has_active_order]", &local_var_str.to_string())]);
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
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListTlsSubsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Change the TLS domains or common name associated with this subscription, update the TLS configuration for this set of domains, or retry a subscription with state `failed` by setting the state to `retry`.
pub async fn patch_tls_sub(configuration: &mut configuration::Configuration, params: PatchTlsSubParams) -> Result<crate::models::TlsSubscriptionResponse, Error<PatchTlsSubError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_subscription_id = params.tls_subscription_id;
    let force = params.force;
    let tls_subscription = params.tls_subscription;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/subscriptions/{tls_subscription_id}", local_var_configuration.base_path, tls_subscription_id=crate::apis::urlencode(tls_subscription_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&tls_subscription);

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
        let local_var_entity: Option<PatchTlsSubError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

