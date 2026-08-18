/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_log_endpoint_errors`]
#[derive(Clone, Debug, Default)]
pub struct GetLogEndpointErrorsParams {
    pub service_id: String,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub filter_endpoint: Option<String>
}


/// struct for typed errors of method [`get_log_endpoint_errors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogEndpointErrorsError {
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Provides a near real-time stream of log errors through a hybrid short-polling model. A client should make an initial request using the `from` parameter to specify a start time. The `to` parameter should be used alongside the `from` parameter since the default bucket is 10 seconds.  For pagination, use the URLs provided in the Link header of the response. These contain updated `from` timestamps for retrieving the next or previous page of logs.  Defaults to `application/x-ndjson` format. Use `Accept: application/json` header to request standard JSON array format instead. 
pub async fn get_log_endpoint_errors(configuration: &mut configuration::Configuration, params: GetLogEndpointErrorsParams) -> Result<String, Error<GetLogEndpointErrorsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let from = params.from;
    let to = params.to;
    let filter_endpoint = params.filter_endpoint;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/observability/service/{service_id}/logging/errors", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_endpoint {
        local_var_req_builder = local_var_req_builder.query(&[("filter[endpoint]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetLogEndpointErrorsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

