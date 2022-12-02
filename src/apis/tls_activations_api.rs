/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_tls_activation`]
#[derive(Clone, Debug, Default)]
pub struct CreateTlsActivationParams {
    pub tls_activation: Option<crate::models::TlsActivation>
}

/// struct for passing parameters to the method [`delete_tls_activation`]
#[derive(Clone, Debug, Default)]
pub struct DeleteTlsActivationParams {
    /// Alphanumeric string identifying a TLS activation.
    pub tls_activation_id: String
}

/// struct for passing parameters to the method [`get_tls_activation`]
#[derive(Clone, Debug, Default)]
pub struct GetTlsActivationParams {
    /// Alphanumeric string identifying a TLS activation.
    pub tls_activation_id: String,
    /// Include related objects. Optional, comma-separated values. Permitted values: `tls_certificate`, `tls_configuration`, and `tls_domain`. 
    pub include: Option<String>
}

/// struct for passing parameters to the method [`list_tls_activations`]
#[derive(Clone, Debug, Default)]
pub struct ListTlsActivationsParams {
    /// Limit the returned activations to a specific certificate.
    pub filter_tls_certificate_id: Option<String>,
    /// Limit the returned activations to a specific TLS configuration.
    pub filter_tls_configuration_id: Option<String>,
    /// Limit the returned rules to a specific domain name.
    pub filter_tls_domain_id: Option<String>,
    /// Include related objects. Optional, comma-separated values. Permitted values: `tls_certificate`, `tls_configuration`, and `tls_domain`. 
    pub include: Option<String>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>
}

/// struct for passing parameters to the method [`update_tls_activation`]
#[derive(Clone, Debug, Default)]
pub struct UpdateTlsActivationParams {
    /// Alphanumeric string identifying a TLS activation.
    pub tls_activation_id: String,
    pub tls_activation: Option<crate::models::TlsActivation>
}


/// struct for typed errors of method [`create_tls_activation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTlsActivationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_tls_activation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTlsActivationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tls_activation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTlsActivationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_tls_activations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTlsActivationsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_tls_activation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTlsActivationError {
    UnknownValue(serde_json::Value),
}


/// Enable TLS for a particular TLS domain and certificate combination. These relationships must be specified to create the TLS activation.
pub async fn create_tls_activation(configuration: &mut configuration::Configuration, params: CreateTlsActivationParams) -> Result<crate::models::TlsActivationResponse, Error<CreateTlsActivationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_activation = params.tls_activation;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/activations", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&tls_activation);

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
        let local_var_entity: Option<CreateTlsActivationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Disable TLS on the domain associated with this TLS activation.
pub async fn delete_tls_activation(configuration: &mut configuration::Configuration, params: DeleteTlsActivationParams) -> Result<(), Error<DeleteTlsActivationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_activation_id = params.tls_activation_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/activations/{tls_activation_id}", local_var_configuration.base_path, tls_activation_id=crate::apis::urlencode(tls_activation_id));
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
        let local_var_entity: Option<DeleteTlsActivationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Show a TLS activation.
pub async fn get_tls_activation(configuration: &mut configuration::Configuration, params: GetTlsActivationParams) -> Result<crate::models::TlsActivationResponse, Error<GetTlsActivationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_activation_id = params.tls_activation_id;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/activations/{tls_activation_id}", local_var_configuration.base_path, tls_activation_id=crate::apis::urlencode(tls_activation_id));
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
        let local_var_entity: Option<GetTlsActivationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all TLS activations.
pub async fn list_tls_activations(configuration: &mut configuration::Configuration, params: ListTlsActivationsParams) -> Result<crate::models::TlsActivationsResponse, Error<ListTlsActivationsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let filter_tls_certificate_id = params.filter_tls_certificate_id;
    let filter_tls_configuration_id = params.filter_tls_configuration_id;
    let filter_tls_domain_id = params.filter_tls_domain_id;
    let include = params.include;
    let page_number = params.page_number;
    let page_size = params.page_size;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/activations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_tls_certificate_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[tls_certificate.id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_tls_configuration_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[tls_configuration.id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_tls_domain_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[tls_domain.id]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListTlsActivationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the certificate used to terminate TLS traffic for the domain associated with this TLS activation.
pub async fn update_tls_activation(configuration: &mut configuration::Configuration, params: UpdateTlsActivationParams) -> Result<crate::models::TlsActivationResponse, Error<UpdateTlsActivationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_activation_id = params.tls_activation_id;
    let tls_activation = params.tls_activation;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/activations/{tls_activation_id}", local_var_configuration.base_path, tls_activation_id=crate::apis::urlencode(tls_activation_id));
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
    local_var_req_builder = local_var_req_builder.json(&tls_activation);

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
        let local_var_entity: Option<UpdateTlsActivationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

