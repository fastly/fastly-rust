/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_service_level_usage`]
#[derive(Clone, Debug, Default)]
pub struct GetServiceLevelUsageParams {
    /// The product identifier for the metrics returned (e.g., `cdn_usage`).
    pub product_id: Option<String>,
    /// The usage type name for the metrics returned (e.g., `North America Requests`).
    pub usage_type_name: Option<String>,
    pub start_month: Option<String>,
    pub end_month: Option<String>,
    /// Number of results per page. The maximum is 100.
    pub limit: Option<String>,
    /// Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty.
    pub cursor: Option<String>
}

/// struct for passing parameters to the method [`get_usage_metrics`]
#[derive(Clone, Debug, Default)]
pub struct GetUsageMetricsParams {
    pub start_month: String,
    pub end_month: String
}


/// struct for typed errors of method [`get_service_level_usage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceLevelUsageError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status500(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_usage_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageMetricsError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status500(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Returns product usage, broken down by service.
pub async fn get_service_level_usage(configuration: &mut configuration::Configuration, params: GetServiceLevelUsageParams) -> Result<crate::models::Serviceusagemetrics, Error<GetServiceLevelUsageError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let product_id = params.product_id;
    let usage_type_name = params.usage_type_name;
    let start_month = params.start_month;
    let end_month = params.end_month;
    let limit = params.limit;
    let cursor = params.cursor;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/billing/v3/service-usage-metrics", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = product_id {
        local_var_req_builder = local_var_req_builder.query(&[("product_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = usage_type_name {
        local_var_req_builder = local_var_req_builder.query(&[("usage_type_name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_month {
        local_var_req_builder = local_var_req_builder.query(&[("start_month", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_month {
        local_var_req_builder = local_var_req_builder.query(&[("end_month", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetServiceLevelUsageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns monthly usage metrics for customer by product.
pub async fn get_usage_metrics(configuration: &mut configuration::Configuration, params: GetUsageMetricsParams) -> Result<crate::models::Usagemetric, Error<GetUsageMetricsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let start_month = params.start_month;
    let end_month = params.end_month;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/billing/v3/usage-metrics", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("start_month", &start_month.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("end_month", &end_month.to_string())]);
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
        let local_var_entity: Option<GetUsageMetricsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

