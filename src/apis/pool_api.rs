/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_server_pool`]
#[derive(Clone, Debug, Default)]
pub struct CreateServerPoolParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// A secure certificate to authenticate a server with. Must be in PEM format.
    pub tls_ca_cert: Option<String>,
    /// The client certificate used to make authenticated requests. Must be in PEM format.
    pub tls_client_cert: Option<String>,
    /// The client private key used to make authenticated requests. Must be in PEM format.
    pub tls_client_key: Option<String>,
    /// The hostname used to verify a server's certificate. It can either be the Common Name (CN) or a Subject Alternative Name (SAN).
    pub tls_cert_hostname: Option<String>,
    /// Whether to use TLS.
    pub use_tls: Option<i32>,
    /// Name for the Pool.
    pub name: Option<String>,
    /// Selected POP to serve as a shield for the servers. Defaults to `null` meaning no origin shielding if not set. Refer to the [POPs API endpoint](/reference/api/utils/pops/) to get a list of available POPs used for shielding.
    pub shield: Option<String>,
    /// Condition which, if met, will select this configuration during a request. Optional.
    pub request_condition: Option<String>,
    /// Maximum number of connections. Optional.
    pub max_conn_default: Option<i32>,
    /// How long to wait for a timeout in milliseconds. Optional.
    pub connect_timeout: Option<i32>,
    /// How long to wait for the first byte in milliseconds. Optional.
    pub first_byte_timeout: Option<i32>,
    /// Percentage of capacity (`0-100`) that needs to be operationally available for a pool to be considered up.
    pub quorum: Option<i32>,
    /// List of OpenSSL ciphers (see the [openssl.org manpages](https://www.openssl.org/docs/man1.1.1/man1/ciphers.html) for details). Optional.
    pub tls_ciphers: Option<String>,
    /// SNI hostname. Optional.
    pub tls_sni_hostname: Option<String>,
    /// Be strict on checking TLS certs. Optional.
    pub tls_check_cert: Option<i32>,
    /// Minimum allowed TLS version on connections to this server. Optional.
    pub min_tls_version: Option<i32>,
    /// Maximum allowed TLS version on connections to this server. Optional.
    pub max_tls_version: Option<i32>,
    /// Name of the healthcheck to use with this pool. Can be empty and could be reused across multiple backend and pools.
    pub healthcheck: Option<String>,
    /// A freeform descriptive note.
    pub comment: Option<String>,
    /// What type of load balance group to use.
    pub _type: Option<String>,
    /// The hostname to [override the Host header](https://docs.fastly.com/en/guides/specifying-an-override-host). Defaults to `null` meaning no override of the Host header will occur. This setting can also be added to a Server definition. If the field is set on a Server definition it will override the Pool setting.
    pub override_host: Option<String>
}

/// struct for passing parameters to the method [`delete_server_pool`]
#[derive(Clone, Debug, Default)]
pub struct DeleteServerPoolParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// Name for the Pool.
    pub pool_name: String
}

/// struct for passing parameters to the method [`get_server_pool`]
#[derive(Clone, Debug, Default)]
pub struct GetServerPoolParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// Name for the Pool.
    pub pool_name: String
}

/// struct for passing parameters to the method [`list_server_pools`]
#[derive(Clone, Debug, Default)]
pub struct ListServerPoolsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32
}

/// struct for passing parameters to the method [`update_server_pool`]
#[derive(Clone, Debug, Default)]
pub struct UpdateServerPoolParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// Name for the Pool.
    pub pool_name: String,
    /// A secure certificate to authenticate a server with. Must be in PEM format.
    pub tls_ca_cert: Option<String>,
    /// The client certificate used to make authenticated requests. Must be in PEM format.
    pub tls_client_cert: Option<String>,
    /// The client private key used to make authenticated requests. Must be in PEM format.
    pub tls_client_key: Option<String>,
    /// The hostname used to verify a server's certificate. It can either be the Common Name (CN) or a Subject Alternative Name (SAN).
    pub tls_cert_hostname: Option<String>,
    /// Whether to use TLS.
    pub use_tls: Option<i32>,
    /// Name for the Pool.
    pub name: Option<String>,
    /// Selected POP to serve as a shield for the servers. Defaults to `null` meaning no origin shielding if not set. Refer to the [POPs API endpoint](/reference/api/utils/pops/) to get a list of available POPs used for shielding.
    pub shield: Option<String>,
    /// Condition which, if met, will select this configuration during a request. Optional.
    pub request_condition: Option<String>,
    /// Maximum number of connections. Optional.
    pub max_conn_default: Option<i32>,
    /// How long to wait for a timeout in milliseconds. Optional.
    pub connect_timeout: Option<i32>,
    /// How long to wait for the first byte in milliseconds. Optional.
    pub first_byte_timeout: Option<i32>,
    /// Percentage of capacity (`0-100`) that needs to be operationally available for a pool to be considered up.
    pub quorum: Option<i32>,
    /// List of OpenSSL ciphers (see the [openssl.org manpages](https://www.openssl.org/docs/man1.1.1/man1/ciphers.html) for details). Optional.
    pub tls_ciphers: Option<String>,
    /// SNI hostname. Optional.
    pub tls_sni_hostname: Option<String>,
    /// Be strict on checking TLS certs. Optional.
    pub tls_check_cert: Option<i32>,
    /// Minimum allowed TLS version on connections to this server. Optional.
    pub min_tls_version: Option<i32>,
    /// Maximum allowed TLS version on connections to this server. Optional.
    pub max_tls_version: Option<i32>,
    /// Name of the healthcheck to use with this pool. Can be empty and could be reused across multiple backend and pools.
    pub healthcheck: Option<String>,
    /// A freeform descriptive note.
    pub comment: Option<String>,
    /// What type of load balance group to use.
    pub _type: Option<String>,
    /// The hostname to [override the Host header](https://docs.fastly.com/en/guides/specifying-an-override-host). Defaults to `null` meaning no override of the Host header will occur. This setting can also be added to a Server definition. If the field is set on a Server definition it will override the Pool setting.
    pub override_host: Option<String>
}


/// struct for typed errors of method [`create_server_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateServerPoolError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_server_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteServerPoolError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_server_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServerPoolError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_server_pools`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListServerPoolsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_server_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateServerPoolError {
    UnknownValue(serde_json::Value),
}


/// Creates a pool for a particular service and version.
pub async fn create_server_pool(configuration: &mut configuration::Configuration, params: CreateServerPoolParams) -> Result<crate::models::PoolResponse, Error<CreateServerPoolError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let tls_ca_cert = params.tls_ca_cert;
    let tls_client_cert = params.tls_client_cert;
    let tls_client_key = params.tls_client_key;
    let tls_cert_hostname = params.tls_cert_hostname;
    let use_tls = params.use_tls;
    let name = params.name;
    let shield = params.shield;
    let request_condition = params.request_condition;
    let max_conn_default = params.max_conn_default;
    let connect_timeout = params.connect_timeout;
    let first_byte_timeout = params.first_byte_timeout;
    let quorum = params.quorum;
    let tls_ciphers = params.tls_ciphers;
    let tls_sni_hostname = params.tls_sni_hostname;
    let tls_check_cert = params.tls_check_cert;
    let min_tls_version = params.min_tls_version;
    let max_tls_version = params.max_tls_version;
    let healthcheck = params.healthcheck;
    let comment = params.comment;
    let _type = params._type;
    let override_host = params.override_host;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/pool", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
    if let Some(local_var_param_value) = tls_ca_cert {
        local_var_form_params.insert("tls_ca_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_client_cert {
        local_var_form_params.insert("tls_client_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_client_key {
        local_var_form_params.insert("tls_client_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_cert_hostname {
        local_var_form_params.insert("tls_cert_hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = use_tls {
        local_var_form_params.insert("use_tls", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = shield {
        local_var_form_params.insert("shield", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = request_condition {
        local_var_form_params.insert("request_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = max_conn_default {
        local_var_form_params.insert("max_conn_default", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = connect_timeout {
        local_var_form_params.insert("connect_timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = first_byte_timeout {
        local_var_form_params.insert("first_byte_timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = quorum {
        local_var_form_params.insert("quorum", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_ciphers {
        local_var_form_params.insert("tls_ciphers", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_sni_hostname {
        local_var_form_params.insert("tls_sni_hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_check_cert {
        local_var_form_params.insert("tls_check_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = min_tls_version {
        local_var_form_params.insert("min_tls_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = max_tls_version {
        local_var_form_params.insert("max_tls_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = healthcheck {
        local_var_form_params.insert("healthcheck", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comment {
        local_var_form_params.insert("comment", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = _type {
        local_var_form_params.insert("type", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = override_host {
        local_var_form_params.insert("override_host", local_var_param_value.to_string());
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
        let local_var_entity: Option<CreateServerPoolError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a specific pool for a particular service and version.
pub async fn delete_server_pool(configuration: &mut configuration::Configuration, params: DeleteServerPoolParams) -> Result<crate::models::InlineResponse200, Error<DeleteServerPoolError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let pool_name = params.pool_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/pool/{pool_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, pool_name=crate::apis::urlencode(pool_name));
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
        let local_var_entity: Option<DeleteServerPoolError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a single pool for a particular service and version.
pub async fn get_server_pool(configuration: &mut configuration::Configuration, params: GetServerPoolParams) -> Result<crate::models::PoolResponse, Error<GetServerPoolError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let pool_name = params.pool_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/pool/{pool_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, pool_name=crate::apis::urlencode(pool_name));
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
        let local_var_entity: Option<GetServerPoolError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all pools for a particular service and pool.
pub async fn list_server_pools(configuration: &mut configuration::Configuration, params: ListServerPoolsParams) -> Result<Vec<crate::models::PoolResponse>, Error<ListServerPoolsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/pool", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
        let local_var_entity: Option<ListServerPoolsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a specific pool for a particular service and version.
pub async fn update_server_pool(configuration: &mut configuration::Configuration, params: UpdateServerPoolParams) -> Result<crate::models::PoolResponse, Error<UpdateServerPoolError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let pool_name = params.pool_name;
    let tls_ca_cert = params.tls_ca_cert;
    let tls_client_cert = params.tls_client_cert;
    let tls_client_key = params.tls_client_key;
    let tls_cert_hostname = params.tls_cert_hostname;
    let use_tls = params.use_tls;
    let name = params.name;
    let shield = params.shield;
    let request_condition = params.request_condition;
    let max_conn_default = params.max_conn_default;
    let connect_timeout = params.connect_timeout;
    let first_byte_timeout = params.first_byte_timeout;
    let quorum = params.quorum;
    let tls_ciphers = params.tls_ciphers;
    let tls_sni_hostname = params.tls_sni_hostname;
    let tls_check_cert = params.tls_check_cert;
    let min_tls_version = params.min_tls_version;
    let max_tls_version = params.max_tls_version;
    let healthcheck = params.healthcheck;
    let comment = params.comment;
    let _type = params._type;
    let override_host = params.override_host;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/pool/{pool_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, pool_name=crate::apis::urlencode(pool_name));
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
    if let Some(local_var_param_value) = tls_ca_cert {
        local_var_form_params.insert("tls_ca_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_client_cert {
        local_var_form_params.insert("tls_client_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_client_key {
        local_var_form_params.insert("tls_client_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_cert_hostname {
        local_var_form_params.insert("tls_cert_hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = use_tls {
        local_var_form_params.insert("use_tls", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = shield {
        local_var_form_params.insert("shield", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = request_condition {
        local_var_form_params.insert("request_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = max_conn_default {
        local_var_form_params.insert("max_conn_default", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = connect_timeout {
        local_var_form_params.insert("connect_timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = first_byte_timeout {
        local_var_form_params.insert("first_byte_timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = quorum {
        local_var_form_params.insert("quorum", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_ciphers {
        local_var_form_params.insert("tls_ciphers", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_sni_hostname {
        local_var_form_params.insert("tls_sni_hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tls_check_cert {
        local_var_form_params.insert("tls_check_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = min_tls_version {
        local_var_form_params.insert("min_tls_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = max_tls_version {
        local_var_form_params.insert("max_tls_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = healthcheck {
        local_var_form_params.insert("healthcheck", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comment {
        local_var_form_params.insert("comment", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = _type {
        local_var_form_params.insert("type", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = override_host {
        local_var_form_params.insert("override_host", local_var_param_value.to_string());
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
        let local_var_entity: Option<UpdateServerPoolError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

