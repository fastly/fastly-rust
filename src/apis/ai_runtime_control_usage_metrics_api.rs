/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`arc_export_usage_metrics`]
#[derive(Clone, Debug, Default)]
pub struct ArcExportUsageMetricsParams {
    /// Filter results to a specific virtual key ID.
    pub key: Option<String>,
    /// Filter results by AI provider connection ID.
    pub provider: Option<String>,
    /// Filter results by AI model identifier.
    pub model: Option<String>,
    /// The start of the time range, in RFC 3339 format.
    pub from: Option<String>,
    /// The end of the time range, in RFC 3339 format. Defaults to now.
    pub to: Option<String>
}

/// struct for passing parameters to the method [`arc_get_usage_metrics`]
#[derive(Clone, Debug, Default)]
pub struct ArcGetUsageMetricsParams {
    /// Filter results to a specific virtual key ID.
    pub key: Option<String>,
    /// Filter results by AI provider connection ID.
    pub provider: Option<String>,
    /// Filter results by AI model identifier.
    pub model: Option<String>,
    /// The start of the time range, in RFC 3339 format.
    pub from: Option<String>,
    /// The end of the time range, in RFC 3339 format. Defaults to now.
    pub to: Option<String>,
    /// The time granularity to aggregate usage by.
    pub granularity: Option<String>,
    /// Sort field. Prefix with `-` for descending order (e.g., `sort=-date`). 
    pub sort: Option<String>
}


/// struct for typed errors of method [`arc_export_usage_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArcExportUsageMetricsError {
    Status400(crate::models::ArcError),
    Status401(crate::models::ArcError),
    Status403(crate::models::ArcError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`arc_get_usage_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArcGetUsageMetricsError {
    Status400(crate::models::ArcError),
    Status401(crate::models::ArcError),
    Status403(crate::models::ArcError),
    UnknownValue(serde_json::Value),
}


/// Returns usage metrics for AI Runtime Control requests. Accepts the same filter parameters as [get usage metrics](#arc-get-usage-metrics), except `sort` and `granularity`, which aren't supported here.  Set the `Accept` header to `text/csv` to receive a CSV file download instead of a JSON response. In the CSV output, the column named `time` corresponds to the `date` field in the JSON response. 
pub async fn arc_export_usage_metrics(configuration: &mut configuration::Configuration, params: ArcExportUsageMetricsParams) -> Result<crate::models::ArcUsageMetricsResponse, Error<ArcExportUsageMetricsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let key = params.key;
    let provider = params.provider;
    let model = params.model;
    let from = params.from;
    let to = params.to;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ai-runtime-control/v1/usage-metrics/export", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = provider {
        local_var_req_builder = local_var_req_builder.query(&[("provider", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = model {
        local_var_req_builder = local_var_req_builder.query(&[("model", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ArcExportUsageMetricsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns usage metrics for AI Runtime Control requests, including request counts, session counts, input token counts, and output token counts.
pub async fn arc_get_usage_metrics(configuration: &mut configuration::Configuration, params: ArcGetUsageMetricsParams) -> Result<crate::models::ArcUsageMetricsResponse, Error<ArcGetUsageMetricsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let key = params.key;
    let provider = params.provider;
    let model = params.model;
    let from = params.from;
    let to = params.to;
    let granularity = params.granularity;
    let sort = params.sort;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ai-runtime-control/v1/usage-metrics", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = provider {
        local_var_req_builder = local_var_req_builder.query(&[("provider", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = model {
        local_var_req_builder = local_var_req_builder.query(&[("model", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = granularity {
        local_var_req_builder = local_var_req_builder.query(&[("granularity", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ArcGetUsageMetricsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

