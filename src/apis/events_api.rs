/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_event`]
#[derive(Clone, Debug, Default)]
pub struct GetEventParams {
    /// Alphanumeric string identifying an event.
    pub event_id: String
}

/// struct for passing parameters to the method [`list_events`]
#[derive(Clone, Debug, Default)]
pub struct ListEventsParams {
    /// Limit the results returned to a specific customer.
    pub filter_customer_id: Option<String>,
    /// Limit the returned events to a specific `event_type`.
    pub filter_event_type: Option<String>,
    /// Limit the results returned to a specific service.
    pub filter_service_id: Option<String>,
    /// Limit the results returned to a specific user.
    pub filter_user_id: Option<String>,
    /// Limit the returned events to a specific token.
    pub filter_token_id: Option<String>,
    /// Limit the returned events to a specific time frame. Accepts sub-parameters: lt, lte, gt, gte (e.g., filter[created_at][gt]=2022-01-12). 
    pub filter_created_at: Option<String>,
    /// Current page.
    pub page_number: Option<i32>,
    /// Number of records per page.
    pub page_size: Option<i32>,
    /// The order in which to list the results by creation date.
    pub sort: Option<String>
}


/// struct for typed errors of method [`get_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEventsError {
    UnknownValue(serde_json::Value),
}


/// Get a specific event.
pub async fn get_event(configuration: &mut configuration::Configuration, params: GetEventParams) -> Result<crate::models::EventResponse, Error<GetEventError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let event_id = params.event_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/events/{event_id}", local_var_configuration.base_path, event_id=crate::apis::urlencode(event_id));
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
        let local_var_entity: Option<GetEventError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all events for a particular customer. Events can be filtered by user, customer and event type. Events can be sorted by date.
pub async fn list_events(configuration: &mut configuration::Configuration, params: ListEventsParams) -> Result<crate::models::EventsResponse, Error<ListEventsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let filter_customer_id = params.filter_customer_id;
    let filter_event_type = params.filter_event_type;
    let filter_service_id = params.filter_service_id;
    let filter_user_id = params.filter_user_id;
    let filter_token_id = params.filter_token_id;
    let filter_created_at = params.filter_created_at;
    let page_number = params.page_number;
    let page_size = params.page_size;
    let sort = params.sort;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/events", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_customer_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[customer_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_event_type {
        local_var_req_builder = local_var_req_builder.query(&[("filter[event_type]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_service_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[service_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_user_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[user_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_token_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[token_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_created_at {
        local_var_req_builder = local_var_req_builder.query(&[("filter[created_at]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListEventsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

