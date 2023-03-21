/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_backend`]
#[derive(Clone, Debug, Default)]
pub struct CreateBackendParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// A hostname, IPv4, or IPv6 address for the backend. This is the preferred way to specify the location of your backend.
    pub address: Option<String>,
    /// Whether or not this backend should be automatically load balanced. If true, all backends with this setting that don't have a `request_condition` will be selected based on their `weight`.
    pub auto_loadbalance: Option<bool>,
    /// Maximum duration in milliseconds that Fastly will wait while receiving no data on a download from a backend. If exceeded, the response received so far will be considered complete and the fetch will end. May be set at runtime using `bereq.between_bytes_timeout`.
    pub between_bytes_timeout: Option<i32>,
    /// Unused.
    pub client_cert: Option<String>,
    /// A freeform descriptive note.
    pub comment: Option<String>,
    /// Maximum duration in milliseconds to wait for a connection to this backend to be established. If exceeded, the connection is aborted and a synthethic `503` response will be presented instead. May be set at runtime using `bereq.connect_timeout`.
    pub connect_timeout: Option<i32>,
    /// Maximum duration in milliseconds to wait for the server response to begin after a TCP connection is established and the request has been sent. If exceeded, the connection is aborted and a synthethic `503` response will be presented instead. May be set at runtime using `bereq.first_byte_timeout`.
    pub first_byte_timeout: Option<i32>,
    /// The name of the healthcheck to use with this backend.
    pub healthcheck: Option<String>,
    /// The hostname of the backend. May be used as an alternative to `address` to set the backend location.
    pub hostname: Option<String>,
    /// IPv4 address of the backend. May be used as an alternative to `address` to set the backend location.
    pub ipv4: Option<String>,
    /// IPv6 address of the backend. May be used as an alternative to `address` to set the backend location.
    pub ipv6: Option<String>,
    /// How long in seconds to keep a persistent connection to the backend between requests.
    pub keepalive_time: Option<i32>,
    /// Maximum number of concurrent connections this backend will accept.
    pub max_conn: Option<i32>,
    /// Maximum allowed TLS version on SSL connections to this backend. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated.
    pub max_tls_version: Option<String>,
    /// Minimum allowed TLS version on SSL connections to this backend. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated.
    pub min_tls_version: Option<String>,
    /// The name of the backend.
    pub name: Option<String>,
    /// If set, will replace the client-supplied HTTP `Host` header on connections to this backend. Applied after VCL has been processed, so this setting will take precedence over changing `bereq.http.Host` in VCL.
    pub override_host: Option<String>,
    /// Port on which the backend server is listening for connections from Fastly. Setting `port` to 80 or 443 will also set `use_ssl` automatically (to false and true respectively), unless explicitly overridden by setting `use_ssl` in the same request.
    pub port: Option<i32>,
    /// Name of a Condition, which if satisfied, will select this backend during a request. If set, will override any `auto_loadbalance` setting. By default, the first backend added to a service is selected for all requests.
    pub request_condition: Option<String>,
    /// Identifier of the POP to use as a [shield](https://docs.fastly.com/en/guides/shielding).
    pub shield: Option<String>,
    /// CA certificate attached to origin.
    pub ssl_ca_cert: Option<String>,
    /// Overrides `ssl_hostname`, but only for cert verification. Does not affect SNI at all.
    pub ssl_cert_hostname: Option<String>,
    /// Be strict on checking SSL certs.
    pub ssl_check_cert: Option<bool>,
    /// List of [OpenSSL ciphers](https://www.openssl.org/docs/manmaster/man1/ciphers.html) to support for connections to this origin. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated.
    pub ssl_ciphers: Option<String>,
    /// Client certificate attached to origin.
    pub ssl_client_cert: Option<String>,
    /// Client key attached to origin.
    pub ssl_client_key: Option<String>,
    /// Use `ssl_cert_hostname` and `ssl_sni_hostname` to configure certificate validation.
    pub ssl_hostname: Option<String>,
    /// Overrides `ssl_hostname`, but only for SNI in the handshake. Does not affect cert validation at all.
    pub ssl_sni_hostname: Option<String>,
    /// Whether or not to require TLS for connections to this backend.
    pub use_ssl: Option<bool>,
    /// Weight used to load balance this backend against others. May be any positive integer. If `auto_loadbalance` is true, the chance of this backend being selected is equal to its own weight over the sum of all weights for backends that have `auto_loadbalance` set to true.
    pub weight: Option<i32>
}

/// struct for passing parameters to the method [`delete_backend`]
#[derive(Clone, Debug, Default)]
pub struct DeleteBackendParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name of the backend.
    pub backend_name: String
}

/// struct for passing parameters to the method [`get_backend`]
#[derive(Clone, Debug, Default)]
pub struct GetBackendParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name of the backend.
    pub backend_name: String
}

/// struct for passing parameters to the method [`list_backends`]
#[derive(Clone, Debug, Default)]
pub struct ListBackendsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32
}

/// struct for passing parameters to the method [`update_backend`]
#[derive(Clone, Debug, Default)]
pub struct UpdateBackendParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name of the backend.
    pub backend_name: String,
    /// A hostname, IPv4, or IPv6 address for the backend. This is the preferred way to specify the location of your backend.
    pub address: Option<String>,
    /// Whether or not this backend should be automatically load balanced. If true, all backends with this setting that don't have a `request_condition` will be selected based on their `weight`.
    pub auto_loadbalance: Option<bool>,
    /// Maximum duration in milliseconds that Fastly will wait while receiving no data on a download from a backend. If exceeded, the response received so far will be considered complete and the fetch will end. May be set at runtime using `bereq.between_bytes_timeout`.
    pub between_bytes_timeout: Option<i32>,
    /// Unused.
    pub client_cert: Option<String>,
    /// A freeform descriptive note.
    pub comment: Option<String>,
    /// Maximum duration in milliseconds to wait for a connection to this backend to be established. If exceeded, the connection is aborted and a synthethic `503` response will be presented instead. May be set at runtime using `bereq.connect_timeout`.
    pub connect_timeout: Option<i32>,
    /// Maximum duration in milliseconds to wait for the server response to begin after a TCP connection is established and the request has been sent. If exceeded, the connection is aborted and a synthethic `503` response will be presented instead. May be set at runtime using `bereq.first_byte_timeout`.
    pub first_byte_timeout: Option<i32>,
    /// The name of the healthcheck to use with this backend.
    pub healthcheck: Option<String>,
    /// The hostname of the backend. May be used as an alternative to `address` to set the backend location.
    pub hostname: Option<String>,
    /// IPv4 address of the backend. May be used as an alternative to `address` to set the backend location.
    pub ipv4: Option<String>,
    /// IPv6 address of the backend. May be used as an alternative to `address` to set the backend location.
    pub ipv6: Option<String>,
    /// How long in seconds to keep a persistent connection to the backend between requests.
    pub keepalive_time: Option<i32>,
    /// Maximum number of concurrent connections this backend will accept.
    pub max_conn: Option<i32>,
    /// Maximum allowed TLS version on SSL connections to this backend. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated.
    pub max_tls_version: Option<String>,
    /// Minimum allowed TLS version on SSL connections to this backend. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated.
    pub min_tls_version: Option<String>,
    /// The name of the backend.
    pub name: Option<String>,
    /// If set, will replace the client-supplied HTTP `Host` header on connections to this backend. Applied after VCL has been processed, so this setting will take precedence over changing `bereq.http.Host` in VCL.
    pub override_host: Option<String>,
    /// Port on which the backend server is listening for connections from Fastly. Setting `port` to 80 or 443 will also set `use_ssl` automatically (to false and true respectively), unless explicitly overridden by setting `use_ssl` in the same request.
    pub port: Option<i32>,
    /// Name of a Condition, which if satisfied, will select this backend during a request. If set, will override any `auto_loadbalance` setting. By default, the first backend added to a service is selected for all requests.
    pub request_condition: Option<String>,
    /// Identifier of the POP to use as a [shield](https://docs.fastly.com/en/guides/shielding).
    pub shield: Option<String>,
    /// CA certificate attached to origin.
    pub ssl_ca_cert: Option<String>,
    /// Overrides `ssl_hostname`, but only for cert verification. Does not affect SNI at all.
    pub ssl_cert_hostname: Option<String>,
    /// Be strict on checking SSL certs.
    pub ssl_check_cert: Option<bool>,
    /// List of [OpenSSL ciphers](https://www.openssl.org/docs/manmaster/man1/ciphers.html) to support for connections to this origin. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated.
    pub ssl_ciphers: Option<String>,
    /// Client certificate attached to origin.
    pub ssl_client_cert: Option<String>,
    /// Client key attached to origin.
    pub ssl_client_key: Option<String>,
    /// Use `ssl_cert_hostname` and `ssl_sni_hostname` to configure certificate validation.
    pub ssl_hostname: Option<String>,
    /// Overrides `ssl_hostname`, but only for SNI in the handshake. Does not affect cert validation at all.
    pub ssl_sni_hostname: Option<String>,
    /// Whether or not to require TLS for connections to this backend.
    pub use_ssl: Option<bool>,
    /// Weight used to load balance this backend against others. May be any positive integer. If `auto_loadbalance` is true, the chance of this backend being selected is equal to its own weight over the sum of all weights for backends that have `auto_loadbalance` set to true.
    pub weight: Option<i32>
}


/// struct for typed errors of method [`create_backend`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateBackendError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_backend`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteBackendError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_backend`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBackendError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_backends`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListBackendsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_backend`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateBackendError {
    UnknownValue(serde_json::Value),
}


/// Create a backend for a particular service and version.
pub async fn create_backend(configuration: &mut configuration::Configuration, params: CreateBackendParams) -> Result<crate::models::BackendResponse, Error<CreateBackendError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let address = params.address;
    let auto_loadbalance = params.auto_loadbalance;
    let between_bytes_timeout = params.between_bytes_timeout;
    let client_cert = params.client_cert;
    let comment = params.comment;
    let connect_timeout = params.connect_timeout;
    let first_byte_timeout = params.first_byte_timeout;
    let healthcheck = params.healthcheck;
    let hostname = params.hostname;
    let ipv4 = params.ipv4;
    let ipv6 = params.ipv6;
    let keepalive_time = params.keepalive_time;
    let max_conn = params.max_conn;
    let max_tls_version = params.max_tls_version;
    let min_tls_version = params.min_tls_version;
    let name = params.name;
    let override_host = params.override_host;
    let port = params.port;
    let request_condition = params.request_condition;
    let shield = params.shield;
    let ssl_ca_cert = params.ssl_ca_cert;
    let ssl_cert_hostname = params.ssl_cert_hostname;
    let ssl_check_cert = params.ssl_check_cert;
    let ssl_ciphers = params.ssl_ciphers;
    let ssl_client_cert = params.ssl_client_cert;
    let ssl_client_key = params.ssl_client_key;
    let ssl_hostname = params.ssl_hostname;
    let ssl_sni_hostname = params.ssl_sni_hostname;
    let use_ssl = params.use_ssl;
    let weight = params.weight;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/backend", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
    if let Some(local_var_param_value) = address {
        local_var_form_params.insert("address", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = auto_loadbalance {
        local_var_form_params.insert("auto_loadbalance", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = between_bytes_timeout {
        local_var_form_params.insert("between_bytes_timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = client_cert {
        local_var_form_params.insert("client_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comment {
        local_var_form_params.insert("comment", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = connect_timeout {
        local_var_form_params.insert("connect_timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = first_byte_timeout {
        local_var_form_params.insert("first_byte_timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = healthcheck {
        local_var_form_params.insert("healthcheck", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = hostname {
        local_var_form_params.insert("hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ipv4 {
        local_var_form_params.insert("ipv4", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ipv6 {
        local_var_form_params.insert("ipv6", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = keepalive_time {
        local_var_form_params.insert("keepalive_time", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = max_conn {
        local_var_form_params.insert("max_conn", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = max_tls_version {
        local_var_form_params.insert("max_tls_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = min_tls_version {
        local_var_form_params.insert("min_tls_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = override_host {
        local_var_form_params.insert("override_host", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = port {
        local_var_form_params.insert("port", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = request_condition {
        local_var_form_params.insert("request_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = shield {
        local_var_form_params.insert("shield", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_ca_cert {
        local_var_form_params.insert("ssl_ca_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_cert_hostname {
        local_var_form_params.insert("ssl_cert_hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_check_cert {
        local_var_form_params.insert("ssl_check_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_ciphers {
        local_var_form_params.insert("ssl_ciphers", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_client_cert {
        local_var_form_params.insert("ssl_client_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_client_key {
        local_var_form_params.insert("ssl_client_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_hostname {
        local_var_form_params.insert("ssl_hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_sni_hostname {
        local_var_form_params.insert("ssl_sni_hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = use_ssl {
        local_var_form_params.insert("use_ssl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = weight {
        local_var_form_params.insert("weight", local_var_param_value.to_string());
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
        let local_var_entity: Option<CreateBackendError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the backend for a particular service and version.
pub async fn delete_backend(configuration: &mut configuration::Configuration, params: DeleteBackendParams) -> Result<crate::models::InlineResponse200, Error<DeleteBackendError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let backend_name = params.backend_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/backend/{backend_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, backend_name=crate::apis::urlencode(backend_name));
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
        let local_var_entity: Option<DeleteBackendError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the backend for a particular service and version.
pub async fn get_backend(configuration: &mut configuration::Configuration, params: GetBackendParams) -> Result<crate::models::BackendResponse, Error<GetBackendError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let backend_name = params.backend_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/backend/{backend_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, backend_name=crate::apis::urlencode(backend_name));
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
        let local_var_entity: Option<GetBackendError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all backends for a particular service and version.
pub async fn list_backends(configuration: &mut configuration::Configuration, params: ListBackendsParams) -> Result<Vec<crate::models::BackendResponse>, Error<ListBackendsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/backend", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
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
        let local_var_entity: Option<ListBackendsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the backend for a particular service and version.
pub async fn update_backend(configuration: &mut configuration::Configuration, params: UpdateBackendParams) -> Result<crate::models::BackendResponse, Error<UpdateBackendError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let backend_name = params.backend_name;
    let address = params.address;
    let auto_loadbalance = params.auto_loadbalance;
    let between_bytes_timeout = params.between_bytes_timeout;
    let client_cert = params.client_cert;
    let comment = params.comment;
    let connect_timeout = params.connect_timeout;
    let first_byte_timeout = params.first_byte_timeout;
    let healthcheck = params.healthcheck;
    let hostname = params.hostname;
    let ipv4 = params.ipv4;
    let ipv6 = params.ipv6;
    let keepalive_time = params.keepalive_time;
    let max_conn = params.max_conn;
    let max_tls_version = params.max_tls_version;
    let min_tls_version = params.min_tls_version;
    let name = params.name;
    let override_host = params.override_host;
    let port = params.port;
    let request_condition = params.request_condition;
    let shield = params.shield;
    let ssl_ca_cert = params.ssl_ca_cert;
    let ssl_cert_hostname = params.ssl_cert_hostname;
    let ssl_check_cert = params.ssl_check_cert;
    let ssl_ciphers = params.ssl_ciphers;
    let ssl_client_cert = params.ssl_client_cert;
    let ssl_client_key = params.ssl_client_key;
    let ssl_hostname = params.ssl_hostname;
    let ssl_sni_hostname = params.ssl_sni_hostname;
    let use_ssl = params.use_ssl;
    let weight = params.weight;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/backend/{backend_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, backend_name=crate::apis::urlencode(backend_name));
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
    if let Some(local_var_param_value) = address {
        local_var_form_params.insert("address", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = auto_loadbalance {
        local_var_form_params.insert("auto_loadbalance", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = between_bytes_timeout {
        local_var_form_params.insert("between_bytes_timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = client_cert {
        local_var_form_params.insert("client_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comment {
        local_var_form_params.insert("comment", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = connect_timeout {
        local_var_form_params.insert("connect_timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = first_byte_timeout {
        local_var_form_params.insert("first_byte_timeout", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = healthcheck {
        local_var_form_params.insert("healthcheck", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = hostname {
        local_var_form_params.insert("hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ipv4 {
        local_var_form_params.insert("ipv4", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ipv6 {
        local_var_form_params.insert("ipv6", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = keepalive_time {
        local_var_form_params.insert("keepalive_time", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = max_conn {
        local_var_form_params.insert("max_conn", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = max_tls_version {
        local_var_form_params.insert("max_tls_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = min_tls_version {
        local_var_form_params.insert("min_tls_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = override_host {
        local_var_form_params.insert("override_host", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = port {
        local_var_form_params.insert("port", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = request_condition {
        local_var_form_params.insert("request_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = shield {
        local_var_form_params.insert("shield", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_ca_cert {
        local_var_form_params.insert("ssl_ca_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_cert_hostname {
        local_var_form_params.insert("ssl_cert_hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_check_cert {
        local_var_form_params.insert("ssl_check_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_ciphers {
        local_var_form_params.insert("ssl_ciphers", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_client_cert {
        local_var_form_params.insert("ssl_client_cert", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_client_key {
        local_var_form_params.insert("ssl_client_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_hostname {
        local_var_form_params.insert("ssl_hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ssl_sni_hostname {
        local_var_form_params.insert("ssl_sni_hostname", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = use_ssl {
        local_var_form_params.insert("use_ssl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = weight {
        local_var_form_params.insert("weight", local_var_param_value.to_string());
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
        let local_var_entity: Option<UpdateBackendError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

