/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_invoice`]
#[derive(Clone, Debug, Default)]
pub struct GetInvoiceParams {
    /// 2-digit month.
    pub month: String,
    /// 4-digit year.
    pub year: String
}

/// struct for passing parameters to the method [`get_invoice_by_id`]
#[derive(Clone, Debug, Default)]
pub struct GetInvoiceByIdParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String,
    /// Alphanumeric string identifying the invoice.
    pub invoice_id: String
}

/// struct for passing parameters to the method [`get_invoice_mtd`]
#[derive(Clone, Debug, Default)]
pub struct GetInvoiceMtdParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String,
    /// 2-digit month.
    pub month: Option<String>,
    /// 4-digit year.
    pub year: Option<String>
}


/// struct for typed errors of method [`get_invoice`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInvoiceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_invoice_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInvoiceByIdError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_invoice_mtd`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInvoiceMtdError {
    UnknownValue(serde_json::Value),
}


/// Get the invoice for a given year and month. Can be any month from when the Customer was created to the current month.
pub async fn get_invoice(configuration: &mut configuration::Configuration, params: GetInvoiceParams) -> Result<crate::models::BillingResponse, Error<GetInvoiceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let month = params.month;
    let year = params.year;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/billing/v2/year/{year}/month/{month}", local_var_configuration.base_path, month=crate::apis::urlencode(month), year=crate::apis::urlencode(year));
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
        let local_var_entity: Option<GetInvoiceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the invoice for the given invoice_id.
pub async fn get_invoice_by_id(configuration: &mut configuration::Configuration, params: GetInvoiceByIdParams) -> Result<crate::models::BillingResponse, Error<GetInvoiceByIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;
    let invoice_id = params.invoice_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/billing/v2/account_customers/{customer_id}/invoices/{invoice_id}", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id), invoice_id=crate::apis::urlencode(invoice_id));
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
        let local_var_entity: Option<GetInvoiceByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the current month-to-date estimate. This endpoint has two different responses. Under normal circumstances, it generally takes less than 5 seconds to generate but in certain cases can take up to 60 seconds. Once generated the month-to-date estimate is cached for 4 hours, and is available the next request will return the JSON representation of the month-to-date estimate. While a report is being generated in the background, this endpoint will return a `202 Accepted` response. The full format of which can be found in detail in our [billing calculation guide](https://docs.fastly.com/en/guides/how-we-calculate-your-bill). There are certain accounts for which we are unable to generate a month-to-date estimate. For example, accounts who have parent-pay are unable to generate an MTD estimate. The parent accounts are able to generate a month-to-date estimate but that estimate will not include the child accounts amounts at this time.
pub async fn get_invoice_mtd(configuration: &mut configuration::Configuration, params: GetInvoiceMtdParams) -> Result<crate::models::BillingEstimateResponse, Error<GetInvoiceMtdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;
    let month = params.month;
    let year = params.year;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/billing/v2/account_customers/{customer_id}/mtd_invoice", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = month {
        local_var_req_builder = local_var_req_builder.query(&[("month", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = year {
        local_var_req_builder = local_var_req_builder.query(&[("year", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetInvoiceMtdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

