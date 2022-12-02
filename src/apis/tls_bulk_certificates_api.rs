/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`delete_bulk_tls_cert`]
#[derive(Clone, Debug, Default)]
pub struct DeleteBulkTlsCertParams {
    /// Alphanumeric string identifying a TLS bulk certificate.
    pub certificate_id: String
}

/// struct for passing parameters to the method [`get_tls_bulk_cert`]
#[derive(Clone, Debug, Default)]
pub struct GetTlsBulkCertParams {
    /// Alphanumeric string identifying a TLS bulk certificate.
    pub certificate_id: String
}

/// struct for passing parameters to the method [`list_tls_bulk_certs`]
#[derive(Clone, Debug, Default)]
pub struct ListTlsBulkCertsParams {
    /// Filter certificates by their matching, fully-qualified domain name.
    pub filter_tls_domain_id: Option<String>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>,
    /// The order in which to list the results by creation date.
    pub sort: Option<String>
}

/// struct for passing parameters to the method [`update_bulk_tls_cert`]
#[derive(Clone, Debug, Default)]
pub struct UpdateBulkTlsCertParams {
    /// Alphanumeric string identifying a TLS bulk certificate.
    pub certificate_id: String,
    pub tls_bulk_certificate: Option<crate::models::TlsBulkCertificate>
}

/// struct for passing parameters to the method [`upload_tls_bulk_cert`]
#[derive(Clone, Debug, Default)]
pub struct UploadTlsBulkCertParams {
    pub tls_bulk_certificate: Option<crate::models::TlsBulkCertificate>
}


/// struct for typed errors of method [`delete_bulk_tls_cert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteBulkTlsCertError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tls_bulk_cert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTlsBulkCertError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_tls_bulk_certs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTlsBulkCertsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_bulk_tls_cert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateBulkTlsCertError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_tls_bulk_cert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadTlsBulkCertError {
    UnknownValue(serde_json::Value),
}


/// Destroy a certificate. This disables TLS for all domains listed as SAN entries.
pub async fn delete_bulk_tls_cert(configuration: &mut configuration::Configuration, params: DeleteBulkTlsCertParams) -> Result<(), Error<DeleteBulkTlsCertError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let certificate_id = params.certificate_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/bulk/certificates/{certificate_id}", local_var_configuration.base_path, certificate_id=crate::apis::urlencode(certificate_id));
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
        let local_var_entity: Option<DeleteBulkTlsCertError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a single certificate.
pub async fn get_tls_bulk_cert(configuration: &mut configuration::Configuration, params: GetTlsBulkCertParams) -> Result<crate::models::TlsBulkCertificateResponse, Error<GetTlsBulkCertError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let certificate_id = params.certificate_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/bulk/certificates/{certificate_id}", local_var_configuration.base_path, certificate_id=crate::apis::urlencode(certificate_id));
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
        let local_var_entity: Option<GetTlsBulkCertError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all certificates.
pub async fn list_tls_bulk_certs(configuration: &mut configuration::Configuration, params: ListTlsBulkCertsParams) -> Result<crate::models::TlsBulkCertificatesResponse, Error<ListTlsBulkCertsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let filter_tls_domain_id = params.filter_tls_domain_id;
    let page_number = params.page_number;
    let page_size = params.page_size;
    let sort = params.sort;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/bulk/certificates", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_tls_domain_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[tls_domain.id]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListTlsBulkCertsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Replace a certificate with a newly reissued certificate. By using this endpoint, the original certificate will cease to be used for future TLS handshakes. Thus, only SAN entries that appear in the replacement certificate will become TLS enabled. Any SAN entries that are missing in the replacement certificate will become disabled.
pub async fn update_bulk_tls_cert(configuration: &mut configuration::Configuration, params: UpdateBulkTlsCertParams) -> Result<crate::models::TlsBulkCertificateResponse, Error<UpdateBulkTlsCertError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let certificate_id = params.certificate_id;
    let tls_bulk_certificate = params.tls_bulk_certificate;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/bulk/certificates/{certificate_id}", local_var_configuration.base_path, certificate_id=crate::apis::urlencode(certificate_id));
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
    local_var_req_builder = local_var_req_builder.json(&tls_bulk_certificate);

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
        let local_var_entity: Option<UpdateBulkTlsCertError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Upload a new certificate. TLS domains are automatically enabled upon certificate creation. If a domain is already enabled on a previously uploaded certificate, that domain will be updated to use the new certificate for all future TLS handshake requests.
pub async fn upload_tls_bulk_cert(configuration: &mut configuration::Configuration, params: UploadTlsBulkCertParams) -> Result<crate::models::TlsBulkCertificateResponse, Error<UploadTlsBulkCertError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let tls_bulk_certificate = params.tls_bulk_certificate;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/bulk/certificates", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&tls_bulk_certificate);

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
        let local_var_entity: Option<UploadTlsBulkCertError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

