/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`add_billing_addr`]
#[derive(Clone, Debug, Default)]
pub struct AddBillingAddrParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String,
    /// Billing address
    pub billing_address_request: Option<crate::models::BillingAddressRequest>
}

/// struct for passing parameters to the method [`delete_billing_addr`]
#[derive(Clone, Debug, Default)]
pub struct DeleteBillingAddrParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String
}

/// struct for passing parameters to the method [`get_billing_addr`]
#[derive(Clone, Debug, Default)]
pub struct GetBillingAddrParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String
}

/// struct for passing parameters to the method [`update_billing_addr`]
#[derive(Clone, Debug, Default)]
pub struct UpdateBillingAddrParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String,
    /// One or more billing address attributes
    pub update_billing_address_request: Option<crate::models::UpdateBillingAddressRequest>
}


/// struct for typed errors of method [`add_billing_addr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddBillingAddrError {
    Status400(crate::models::BillingAddressVerificationErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_billing_addr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteBillingAddrError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_billing_addr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBillingAddrError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_billing_addr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateBillingAddrError {
    Status400(crate::models::BillingAddressVerificationErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Add a billing address to a customer.
pub async fn add_billing_addr(configuration: &mut configuration::Configuration, params: AddBillingAddrParams) -> Result<crate::models::BillingAddressResponse, Error<AddBillingAddrError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;
    let billing_address_request = params.billing_address_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/customer/{customer_id}/billing_address", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&billing_address_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "POST" != "GET" && "POST" != "HEAD" {
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
        let local_var_entity: Option<AddBillingAddrError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a customer's billing address.
pub async fn delete_billing_addr(configuration: &mut configuration::Configuration, params: DeleteBillingAddrParams) -> Result<(), Error<DeleteBillingAddrError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/customer/{customer_id}/billing_address", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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

    if "DELETE" != "GET" && "DELETE" != "HEAD" {
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
        Ok(())
    } else {
        let local_var_entity: Option<DeleteBillingAddrError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a customer's billing address.
pub async fn get_billing_addr(configuration: &mut configuration::Configuration, params: GetBillingAddrParams) -> Result<crate::models::BillingAddressResponse, Error<GetBillingAddrError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/customer/{customer_id}/billing_address", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id));
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
        let local_var_entity: Option<GetBillingAddrError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a customer's billing address. You may update only part of the customer's billing address.
pub async fn update_billing_addr(configuration: &mut configuration::Configuration, params: UpdateBillingAddrParams) -> Result<crate::models::BillingAddressResponse, Error<UpdateBillingAddrError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;
    let update_billing_address_request = params.update_billing_address_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/customer/{customer_id}/billing_address", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&update_billing_address_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "PATCH" != "GET" && "PATCH" != "HEAD" {
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
        let local_var_entity: Option<UpdateBillingAddrError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

