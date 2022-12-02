/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_tls_config`]
#[derive(Clone, Debug, Default)]
pub struct GetTlsConfigParams {
    /// Alphanumeric string identifying a TLS configuration.
    pub tls_configuration_id: String,
    /// Include related objects. Optional, comma-separated values. Permitted values: `dns_records`. 
    pub include: Option<String>
}

/// struct for passing parameters to the method [`list_tls_configs`]
#[derive(Clone, Debug, Default)]
pub struct ListTlsConfigsParams {
    /// Optionally filters by the bulk attribute.
    pub filter_bulk: Option<String>,
    /// Include related objects. Optional, comma-separated values. Permitted values: `dns_records`. 
    pub include: Option<String>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>
}

/// struct for passing parameters to the method [`update_tls_config`]
#[derive(Clone, Debug, Default)]
pub struct UpdateTlsConfigParams {
    /// Alphanumeric string identifying a TLS configuration.
    pub tls_configuration_id: String,
    pub tls_configuration: Option<crate::models::TlsConfiguration>
}


/// struct for typed errors of method [`get_tls_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTlsConfigError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_tls_configs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTlsConfigsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_tls_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTlsConfigError {
    UnknownValue(serde_json::Value),
}


/// Show a TLS configuration.
pub async fn get_tls_config(configuration: &mut configuration::Configuration, params: GetTlsConfigParams) -> Result<crate::models::TlsConfigurationResponse, Error<GetTlsConfigError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_configuration_id = params.tls_configuration_id;
    let include = params.include;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/configurations/{tls_configuration_id}", local_var_configuration.base_path, tls_configuration_id=crate::apis::urlencode(tls_configuration_id));
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
        let local_var_entity: Option<GetTlsConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all TLS configurations.
pub async fn list_tls_configs(configuration: &mut configuration::Configuration, params: ListTlsConfigsParams) -> Result<crate::models::TlsConfigurationsResponse, Error<ListTlsConfigsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let filter_bulk = params.filter_bulk;
    let include = params.include;
    let page_number = params.page_number;
    let page_size = params.page_size;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/configurations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_bulk {
        local_var_req_builder = local_var_req_builder.query(&[("filter[bulk]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListTlsConfigsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a TLS configuration.
pub async fn update_tls_config(configuration: &mut configuration::Configuration, params: UpdateTlsConfigParams) -> Result<crate::models::TlsConfigurationResponse, Error<UpdateTlsConfigError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_configuration_id = params.tls_configuration_id;
    let tls_configuration = params.tls_configuration;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/configurations/{tls_configuration_id}", local_var_configuration.base_path, tls_configuration_id=crate::apis::urlencode(tls_configuration_id));
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
    local_var_req_builder = local_var_req_builder.json(&tls_configuration);

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
        let local_var_entity: Option<UpdateTlsConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

