/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_service_stats`]
#[derive(Clone, Debug, Default)]
pub struct GetServiceStatsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// 2-digit month.
    pub month: Option<String>,
    /// 4-digit year.
    pub year: Option<String>,
    /// Epoch timestamp. Limits the results returned.
    pub start_time: Option<i32>,
    /// Epoch timestamp. Limits the results returned.
    pub end_time: Option<i32>
}


/// struct for typed errors of method [`get_service_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceStatsError {
    UnknownValue(serde_json::Value),
}


/// Get the stats from a service for a block of time. This lists all stats by PoP location, starting with AMS. This call requires parameters to select block of time to query. Use either a timestamp range (using start_time and end_time) or a specified month/year combo (using month and year).
pub async fn get_service_stats(configuration: &mut configuration::Configuration, params: GetServiceStatsParams) -> Result<crate::models::Stats, Error<GetServiceStatsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let month = params.month;
    let year = params.year;
    let start_time = params.start_time;
    let end_time = params.end_time;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/stats/summary", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = month {
        local_var_req_builder = local_var_req_builder.query(&[("month", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = year {
        local_var_req_builder = local_var_req_builder.query(&[("year", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_time {
        local_var_req_builder = local_var_req_builder.query(&[("start_time", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_time {
        local_var_req_builder = local_var_req_builder.query(&[("end_time", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetServiceStatsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

