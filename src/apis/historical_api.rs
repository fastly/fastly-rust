/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_hist_stats`]
#[derive(Clone, Debug, Default)]
pub struct GetHistStatsParams {
    /// Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`. 
    pub from: Option<String>,
    /// Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`. 
    pub to: Option<String>,
    /// Duration of sample windows. One of:   * `hour` - Group data by hour.   * `minute` - Group data by minute.   * `day` - Group data by day. 
    pub by: Option<String>,
    /// Limit query to a specific geographic region. One of:   * `usa` - North America.   * `europe` - Europe.   * `anzac` - Australia and New Zealand.   * `asia` - Asia.   * `asia_india` - India.   * `asia_southkorea` - South Korea.   * `africa_std` - Africa.   * `southamerica_std` - South America. 
    pub region: Option<String>
}

/// struct for passing parameters to the method [`get_hist_stats_aggregated`]
#[derive(Clone, Debug, Default)]
pub struct GetHistStatsAggregatedParams {
    /// Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`. 
    pub from: Option<String>,
    /// Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`. 
    pub to: Option<String>,
    /// Duration of sample windows. One of:   * `hour` - Group data by hour.   * `minute` - Group data by minute.   * `day` - Group data by day. 
    pub by: Option<String>,
    /// Limit query to a specific geographic region. One of:   * `usa` - North America.   * `europe` - Europe.   * `anzac` - Australia and New Zealand.   * `asia` - Asia.   * `asia_india` - India.   * `asia_southkorea` - South Korea.   * `africa_std` - Africa.   * `southamerica_std` - South America. 
    pub region: Option<String>
}

/// struct for passing parameters to the method [`get_hist_stats_field`]
#[derive(Clone, Debug, Default)]
pub struct GetHistStatsFieldParams {
    /// Name of the stats field.
    pub field: String,
    /// Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`. 
    pub from: Option<String>,
    /// Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`. 
    pub to: Option<String>,
    /// Duration of sample windows. One of:   * `hour` - Group data by hour.   * `minute` - Group data by minute.   * `day` - Group data by day. 
    pub by: Option<String>,
    /// Limit query to a specific geographic region. One of:   * `usa` - North America.   * `europe` - Europe.   * `anzac` - Australia and New Zealand.   * `asia` - Asia.   * `asia_india` - India.   * `asia_southkorea` - South Korea.   * `africa_std` - Africa.   * `southamerica_std` - South America. 
    pub region: Option<String>
}

/// struct for passing parameters to the method [`get_hist_stats_service`]
#[derive(Clone, Debug, Default)]
pub struct GetHistStatsServiceParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`. 
    pub from: Option<String>,
    /// Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`. 
    pub to: Option<String>,
    /// Duration of sample windows. One of:   * `hour` - Group data by hour.   * `minute` - Group data by minute.   * `day` - Group data by day. 
    pub by: Option<String>,
    /// Limit query to a specific geographic region. One of:   * `usa` - North America.   * `europe` - Europe.   * `anzac` - Australia and New Zealand.   * `asia` - Asia.   * `asia_india` - India.   * `asia_southkorea` - South Korea.   * `africa_std` - Africa.   * `southamerica_std` - South America. 
    pub region: Option<String>
}

/// struct for passing parameters to the method [`get_hist_stats_service_field`]
#[derive(Clone, Debug, Default)]
pub struct GetHistStatsServiceFieldParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Name of the stats field.
    pub field: String,
    /// Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`. 
    pub from: Option<String>,
    /// Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`. 
    pub to: Option<String>,
    /// Duration of sample windows. One of:   * `hour` - Group data by hour.   * `minute` - Group data by minute.   * `day` - Group data by day. 
    pub by: Option<String>,
    /// Limit query to a specific geographic region. One of:   * `usa` - North America.   * `europe` - Europe.   * `anzac` - Australia and New Zealand.   * `asia` - Asia.   * `asia_india` - India.   * `asia_southkorea` - South Korea.   * `africa_std` - Africa.   * `southamerica_std` - South America. 
    pub region: Option<String>
}

/// struct for passing parameters to the method [`get_usage`]
#[derive(Clone, Debug, Default)]
pub struct GetUsageParams {
    /// Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`. 
    pub from: Option<String>,
    /// Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`. 
    pub to: Option<String>
}

/// struct for passing parameters to the method [`get_usage_month`]
#[derive(Clone, Debug, Default)]
pub struct GetUsageMonthParams {
    /// 4-digit year.
    pub year: Option<String>,
    /// 2-digit month.
    pub month: Option<String>,
    /// If `true`, return results as billable units.
    pub billable_units: Option<bool>
}

/// struct for passing parameters to the method [`get_usage_service`]
#[derive(Clone, Debug, Default)]
pub struct GetUsageServiceParams {
    /// Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`. 
    pub from: Option<String>,
    /// Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`. 
    pub to: Option<String>
}


/// struct for typed errors of method [`get_hist_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHistStatsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_hist_stats_aggregated`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHistStatsAggregatedError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_hist_stats_field`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHistStatsFieldError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_hist_stats_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHistStatsServiceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_hist_stats_service_field`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHistStatsServiceFieldError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_regions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRegionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_usage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_usage_month`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageMonthError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_usage_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsageServiceError {
    UnknownValue(serde_json::Value),
}


/// Fetches historical stats for each of your Fastly services and groups the results by service ID.
pub async fn get_hist_stats(configuration: &mut configuration::Configuration, params: GetHistStatsParams) -> Result<crate::models::HistoricalResponse, Error<GetHistStatsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let from = params.from;
    let to = params.to;
    let by = params.by;
    let region = params.region;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stats", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = by {
        local_var_req_builder = local_var_req_builder.query(&[("by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetHistStatsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetches historical stats information aggregated across all of your Fastly services.
pub async fn get_hist_stats_aggregated(configuration: &mut configuration::Configuration, params: GetHistStatsAggregatedParams) -> Result<crate::models::HistoricalAggregateResponse, Error<GetHistStatsAggregatedError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let from = params.from;
    let to = params.to;
    let by = params.by;
    let region = params.region;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stats/aggregate", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = by {
        local_var_req_builder = local_var_req_builder.query(&[("by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetHistStatsAggregatedError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetches the specified field from the historical stats for each of your services and groups the results by service ID.
pub async fn get_hist_stats_field(configuration: &mut configuration::Configuration, params: GetHistStatsFieldParams) -> Result<crate::models::HistoricalFieldResponse, Error<GetHistStatsFieldError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let field = params.field;
    let from = params.from;
    let to = params.to;
    let by = params.by;
    let region = params.region;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stats/field/{field}", local_var_configuration.base_path, field=crate::apis::urlencode(field));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = by {
        local_var_req_builder = local_var_req_builder.query(&[("by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetHistStatsFieldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetches historical stats for a given service.
pub async fn get_hist_stats_service(configuration: &mut configuration::Configuration, params: GetHistStatsServiceParams) -> Result<crate::models::HistoricalAggregateResponse, Error<GetHistStatsServiceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let from = params.from;
    let to = params.to;
    let by = params.by;
    let region = params.region;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stats/service/{service_id}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = by {
        local_var_req_builder = local_var_req_builder.query(&[("by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetHistStatsServiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetches the specified field from the historical stats for a given service.
pub async fn get_hist_stats_service_field(configuration: &mut configuration::Configuration, params: GetHistStatsServiceFieldParams) -> Result<crate::models::HistoricalFieldAggregateResponse, Error<GetHistStatsServiceFieldError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let field = params.field;
    let from = params.from;
    let to = params.to;
    let by = params.by;
    let region = params.region;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stats/service/{service_id}/field/{field}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), field=crate::apis::urlencode(field));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = by {
        local_var_req_builder = local_var_req_builder.query(&[("by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = region {
        local_var_req_builder = local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetHistStatsServiceFieldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetches the list of codes for regions that are covered by the Fastly CDN service.
pub async fn get_regions(configuration: &mut configuration::Configuration) -> Result<crate::models::HistoricalRegionsResponse, Error<GetRegionsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stats/regions", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetRegionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns usage information aggregated across all Fastly services and grouped by region. To aggregate across all Fastly services by time period, see [`/stats/aggregate`](#get-hist-stats-aggregated).
pub async fn get_usage(configuration: &mut configuration::Configuration, params: GetUsageParams) -> Result<crate::models::HistoricalUsageAggregateResponse, Error<GetUsageError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let from = params.from;
    let to = params.to;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stats/usage", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetUsageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns month-to-date usage details for a given month and year. Usage details are aggregated by service and across all Fastly services, and then grouped by region. This endpoint does not use the `from` or `to` fields for selecting the date for which data is requested. Instead, it uses `month` and `year` integer fields. Both fields are optional and default to the current month and year respectively. When set, an optional `billable_units` field will convert bandwidth to GB and divide requests by 10,000.
pub async fn get_usage_month(configuration: &mut configuration::Configuration, params: GetUsageMonthParams) -> Result<crate::models::HistoricalUsageMonthResponse, Error<GetUsageMonthError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let year = params.year;
    let month = params.month;
    let billable_units = params.billable_units;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stats/usage_by_month", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = year {
        local_var_req_builder = local_var_req_builder.query(&[("year", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = month {
        local_var_req_builder = local_var_req_builder.query(&[("month", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = billable_units {
        local_var_req_builder = local_var_req_builder.query(&[("billable_units", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetUsageMonthError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns usage information aggregated by service and grouped by service and region. For service stats by time period, see [`/stats`](#get-hist-stats) and [`/stats/field/:field`](#get-hist-stats-field).
pub async fn get_usage_service(configuration: &mut configuration::Configuration, params: GetUsageServiceParams) -> Result<crate::models::HistoricalUsageServiceResponse, Error<GetUsageServiceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let from = params.from;
    let to = params.to;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/stats/usage_by_service", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetUsageServiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

