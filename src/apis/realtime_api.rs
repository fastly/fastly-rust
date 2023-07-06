/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_stats_last120_seconds`]
#[derive(Clone, Debug, Default)]
pub struct GetStatsLast120SecondsParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String
}

/// struct for passing parameters to the method [`get_stats_last120_seconds_limit_entries`]
#[derive(Clone, Debug, Default)]
pub struct GetStatsLast120SecondsLimitEntriesParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Maximum number of results to show.
    pub max_entries: i32
}

/// struct for passing parameters to the method [`get_stats_last_second`]
#[derive(Clone, Debug, Default)]
pub struct GetStatsLastSecondParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Timestamp in seconds (Unix epoch time).
    pub timestamp_in_seconds: i32
}


/// struct for typed errors of method [`get_stats_last120_seconds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStatsLast120SecondsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_stats_last120_seconds_limit_entries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStatsLast120SecondsLimitEntriesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_stats_last_second`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStatsLastSecondError {
    UnknownValue(serde_json::Value),
}


/// Get data for the 120 seconds preceding the latest timestamp available for a service.
pub async fn get_stats_last120_seconds(configuration: &mut configuration::Configuration, params: GetStatsLast120SecondsParams) -> Result<crate::models::Realtime, Error<GetStatsLast120SecondsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/channel/{service_id}/ts/h", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id));
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
        let local_var_entity: Option<GetStatsLast120SecondsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get data for the 120 seconds preceding the latest timestamp available for a service, up to a maximum of `max_entries` entries.
pub async fn get_stats_last120_seconds_limit_entries(configuration: &mut configuration::Configuration, params: GetStatsLast120SecondsLimitEntriesParams) -> Result<crate::models::Realtime, Error<GetStatsLast120SecondsLimitEntriesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let max_entries = params.max_entries;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/channel/{service_id}/ts/h/limit/{max_entries}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), max_entries=max_entries);
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
        let local_var_entity: Option<GetStatsLast120SecondsLimitEntriesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get real-time data for the specified reporting period. Specify `0` to get a single entry for the last complete second. The `Timestamp` field included in the response provides the time index of the latest entry in the dataset and can be provided as the `start_timestamp` of the next request for a seamless continuation of the dataset from one request to the next.
pub async fn get_stats_last_second(configuration: &mut configuration::Configuration, params: GetStatsLastSecondParams) -> Result<crate::models::Realtime, Error<GetStatsLastSecondError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let timestamp_in_seconds = params.timestamp_in_seconds;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/channel/{service_id}/ts/{timestamp_in_seconds}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), timestamp_in_seconds=timestamp_in_seconds);
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
        let local_var_entity: Option<GetStatsLastSecondError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

