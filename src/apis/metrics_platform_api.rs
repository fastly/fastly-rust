/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_platform_metrics_service_historical`]
#[derive(Clone, Debug, Default)]
pub struct GetPlatformMetricsServiceHistoricalParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Duration of sample windows.
    pub granularity: String,
    /// A valid RFC-8339-formatted date and time indicating the inclusive start of the query time range. If not provided, a default is chosen based on the provided `granularity` value.
    pub from: Option<String>,
    /// A valid RFC-8339-formatted date and time indicating the exclusive end of the query time range. If not provided, a default is chosen based on the provided `granularity` value.
    pub to: Option<String>,
    /// The metric(s) to retrieve. Multiple values should be comma-separated.
    pub metric: Option<String>,
    /// The metric set(s) to retrieve. Multiple values should be comma-separated.
    pub metric_set: Option<String>,
    /// Field to group_by in the query. For example, `group_by=region` will return entries for grouped by timestamp and region. 
    pub group_by: Option<String>,
    /// Limit query to one or more specific geographic regions. Values should be comma-separated. 
    pub region: Option<String>,
    /// Limit query to one or more specific POPs. Values should be comma-separated.
    pub datacenter: Option<String>,
    /// Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty.
    pub cursor: Option<String>,
    /// Number of results per page. The maximum is 10000.
    pub limit: Option<String>
}


/// struct for typed errors of method [`get_platform_metrics_service_historical`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPlatformMetricsServiceHistoricalError {
    UnknownValue(serde_json::Value),
}


/// Fetches historical metrics for a single service for a given granularity.
pub async fn get_platform_metrics_service_historical(configuration: &mut configuration::Configuration, params: GetPlatformMetricsServiceHistoricalParams) -> Result<crate::models::PlatformMetricsResponse, Error<GetPlatformMetricsServiceHistoricalError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let granularity = params.granularity;
    let from = params.from;
    let to = params.to;
    let metric = params.metric;
    let metric_set = params.metric_set;
    let group_by = params.group_by;
    let region = params.region;
    let datacenter = params.datacenter;
    let cursor = params.cursor;
    let limit = params.limit;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/metrics/platform/services/{service_id}/{granularity}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), granularity=crate::apis::urlencode(granularity));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = metric {
        local_var_req_builder = local_var_req_builder.query(&[("metric", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = metric_set {
        local_var_req_builder = local_var_req_builder.query(&[("metric_set", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = group_by {
        local_var_req_builder = local_var_req_builder.query(&[("group_by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = datacenter {
        local_var_req_builder = local_var_req_builder.query(&[("datacenter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetPlatformMetricsServiceHistoricalError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

