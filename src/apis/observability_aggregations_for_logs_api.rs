/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`log_aggregations_get`]
#[derive(Clone, Debug, Default)]
pub struct LogAggregationsGetParams {
    pub source: String,
    pub service_id: String,
    pub start: String,
    pub end: String,
    pub series: String,
    pub limit: Option<f32>,
    pub filter: Option<String>,
    pub dimensions: Option<String>,
    pub sort: Option<String>
}


/// struct for typed errors of method [`log_aggregations_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogAggregationsGetError {
    UnknownValue(serde_json::Value),
}


/// Retrieves aggregated log results.
pub async fn log_aggregations_get(configuration: &mut configuration::Configuration, params: LogAggregationsGetParams) -> Result<crate::models::LogAggregationsGetResponse, Error<LogAggregationsGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let source = params.source;
    let service_id = params.service_id;
    let start = params.start;
    let end = params.end;
    let series = params.series;
    let limit = params.limit;
    let filter = params.filter;
    let dimensions = params.dimensions;
    let sort = params.sort;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/observability/aggregations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("source", &source.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("service_id", &service_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("start", &start.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("end", &end.to_string())]);
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter {
        local_var_req_builder = local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("series", &series.to_string())]);
    if let Some(ref local_var_str) = dimensions {
        local_var_req_builder = local_var_req_builder.query(&[("dimensions", &local_var_str.to_string())]);
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
        let local_var_entity: Option<LogAggregationsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

