/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_tls_cert`]
#[derive(Clone, Debug, Default)]
pub struct CreateTlsCertParams {
    pub tls_certificate: Option<crate::models::TlsCertificate>
}

/// struct for passing parameters to the method [`delete_tls_cert`]
#[derive(Clone, Debug, Default)]
pub struct DeleteTlsCertParams {
    /// Alphanumeric string identifying a TLS certificate.
    pub tls_certificate_id: String
}

/// struct for passing parameters to the method [`get_tls_cert`]
#[derive(Clone, Debug, Default)]
pub struct GetTlsCertParams {
    /// Alphanumeric string identifying a TLS certificate.
    pub tls_certificate_id: String
}

/// struct for passing parameters to the method [`get_tls_cert_blob`]
#[derive(Clone, Debug, Default)]
pub struct GetTlsCertBlobParams {
    /// Alphanumeric string identifying a TLS certificate.
    pub tls_certificate_id: String
}

/// struct for passing parameters to the method [`list_tls_certs`]
#[derive(Clone, Debug, Default)]
pub struct ListTlsCertsParams {
    /// Optional. Limit the returned certificates to those currently using Fastly to terminate TLS (that is, certificates associated with an activation). Permitted values: true, false.
    pub filter_in_use: Option<String>,
    /// Limit the returned certificates to those that expire prior to the specified date in UTC. Accepts parameters: lte (e.g., filter[not_after][lte]=2020-05-05). 
    pub filter_not_after: Option<String>,
    /// Limit the returned certificates to those that include the specific domain.
    pub filter_tls_domains_id: Option<String>,
    /// Include related objects. Optional, comma-separated values. Permitted values: `tls_activations`. 
    pub include: Option<String>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>,
    /// The order in which to list the results by creation date.
    pub sort: Option<String>
}

/// struct for passing parameters to the method [`update_tls_cert`]
#[derive(Clone, Debug, Default)]
pub struct UpdateTlsCertParams {
    /// Alphanumeric string identifying a TLS certificate.
    pub tls_certificate_id: String,
    pub tls_certificate: Option<crate::models::TlsCertificate>
}


/// struct for typed errors of method [`create_tls_cert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTlsCertError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_tls_cert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTlsCertError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tls_cert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTlsCertError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tls_cert_blob`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTlsCertBlobError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_tls_certs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTlsCertsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_tls_cert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTlsCertError {
    UnknownValue(serde_json::Value),
}


/// Create a TLS certificate.
pub async fn create_tls_cert(configuration: &mut configuration::Configuration, params: CreateTlsCertParams) -> Result<serde_json::Value, Error<CreateTlsCertError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_certificate = params.tls_certificate;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/certificates", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&tls_certificate);

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
        let local_var_entity: Option<CreateTlsCertError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Destroy a TLS certificate. TLS certificates already enabled for a domain cannot be destroyed.
pub async fn delete_tls_cert(configuration: &mut configuration::Configuration, params: DeleteTlsCertParams) -> Result<(), Error<DeleteTlsCertError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_certificate_id = params.tls_certificate_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/certificates/{tls_certificate_id}", local_var_configuration.base_path, tls_certificate_id=crate::apis::urlencode(tls_certificate_id));
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
        let local_var_entity: Option<DeleteTlsCertError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Show a TLS certificate.
pub async fn get_tls_cert(configuration: &mut configuration::Configuration, params: GetTlsCertParams) -> Result<crate::models::TlsCertificateResponse, Error<GetTlsCertError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_certificate_id = params.tls_certificate_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/certificates/{tls_certificate_id}", local_var_configuration.base_path, tls_certificate_id=crate::apis::urlencode(tls_certificate_id));
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
        let local_var_entity: Option<GetTlsCertError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a TLS certificate blob. This feature is part of a [limited availability](https://docs.fastly.com/products/fastly-product-lifecycle#limited-availability) release.
pub async fn get_tls_cert_blob(configuration: &mut configuration::Configuration, params: GetTlsCertBlobParams) -> Result<crate::models::TlsCertificateBlobResponse, Error<GetTlsCertBlobError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_certificate_id = params.tls_certificate_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/certificates/{tls_certificate_id}/blob", local_var_configuration.base_path, tls_certificate_id=crate::apis::urlencode(tls_certificate_id));
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
        let local_var_entity: Option<GetTlsCertBlobError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all TLS certificates.
pub async fn list_tls_certs(configuration: &mut configuration::Configuration, params: ListTlsCertsParams) -> Result<crate::models::TlsCertificatesResponse, Error<ListTlsCertsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let filter_in_use = params.filter_in_use;
    let filter_not_after = params.filter_not_after;
    let filter_tls_domains_id = params.filter_tls_domains_id;
    let include = params.include;
    let page_number = params.page_number;
    let page_size = params.page_size;
    let sort = params.sort;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/certificates", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_in_use {
        local_var_req_builder = local_var_req_builder.query(&[("filter[in_use]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_not_after {
        local_var_req_builder = local_var_req_builder.query(&[("filter[not_after]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_tls_domains_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[tls_domains.id]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListTlsCertsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Replace a TLS certificate with a newly reissued TLS certificate, or update a TLS certificate's name. If replacing a TLS certificate, the new TLS certificate must contain all SAN entries as the current TLS certificate. It must either have an exact matching list or contain a superset.
pub async fn update_tls_cert(configuration: &mut configuration::Configuration, params: UpdateTlsCertParams) -> Result<crate::models::TlsCertificateResponse, Error<UpdateTlsCertError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_certificate_id = params.tls_certificate_id;
    let tls_certificate = params.tls_certificate;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/certificates/{tls_certificate_id}", local_var_configuration.base_path, tls_certificate_id=crate::apis::urlencode(tls_certificate_id));
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
    local_var_req_builder = local_var_req_builder.json(&tls_certificate);

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
        let local_var_entity: Option<UpdateTlsCertError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

