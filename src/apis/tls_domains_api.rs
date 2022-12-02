/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`list_tls_domains`]
#[derive(Clone, Debug, Default)]
pub struct ListTlsDomainsParams {
    /// Optional. Limit the returned domains to those currently using Fastly to terminate TLS with SNI (that is, domains considered \"in use\") Permitted values: true, false.
    pub filter_in_use: Option<String>,
    /// Optional. Limit the returned domains to those listed in the given TLS certificate's SAN list.
    pub filter_tls_certificates_id: Option<String>,
    /// Optional. Limit the returned domains to those for a given TLS subscription.
    pub filter_tls_subscriptions_id: Option<String>,
    /// Include related objects. Optional, comma-separated values. Permitted values: `tls_activations`, `tls_certificates`, `tls_subscriptions`, `tls_subscriptions.tls_authorizations`, and `tls_authorizations.globalsign_email_challenge`. 
    pub include: Option<String>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>,
    /// The order in which to list the results by creation date.
    pub sort: Option<String>
}


/// struct for typed errors of method [`list_tls_domains`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTlsDomainsError {
    UnknownValue(serde_json::Value),
}


/// List all TLS domains.
pub async fn list_tls_domains(configuration: &mut configuration::Configuration, params: ListTlsDomainsParams) -> Result<crate::models::TlsDomainsResponse, Error<ListTlsDomainsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let filter_in_use = params.filter_in_use;
    let filter_tls_certificates_id = params.filter_tls_certificates_id;
    let filter_tls_subscriptions_id = params.filter_tls_subscriptions_id;
    let include = params.include;
    let page_number = params.page_number;
    let page_size = params.page_size;
    let sort = params.sort;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tls/domains", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_in_use {
        local_var_req_builder = local_var_req_builder.query(&[("filter[in_use]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_tls_certificates_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[tls_certificates.id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_tls_subscriptions_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[tls_subscriptions.id]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListTlsDomainsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

