/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_contacts`]
#[derive(Clone, Debug, Default)]
pub struct CreateContactsParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String,
    /// The alphanumeric string representing the user for this customer contact.
    pub user_id: Option<String>,
    /// The type of contact.
    pub contact_type: Option<String>,
    /// The name of this contact, when user_id is not provided.
    pub name: Option<String>,
    /// The email of this contact, when a user_id is not provided.
    pub email: Option<String>,
    /// The phone number for this contact. Required for primary, technical, and security contact types.
    pub phone: Option<String>,
    /// The alphanumeric string representing the customer for this customer contact.
    pub customer_id2: Option<String>
}

/// struct for passing parameters to the method [`delete_contact`]
#[derive(Clone, Debug, Default)]
pub struct DeleteContactParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String,
    /// An alphanumeric string identifying the customer contact.
    pub contact_id: String
}

/// struct for passing parameters to the method [`list_contacts`]
#[derive(Clone, Debug, Default)]
pub struct ListContactsParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String
}


/// struct for typed errors of method [`create_contacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateContactsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_contact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteContactError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_contacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListContactsError {
    UnknownValue(serde_json::Value),
}


/// Create a contact.
pub async fn create_contacts(configuration: &mut configuration::Configuration, params: CreateContactsParams) -> Result<crate::models::ContactResponse, Error<CreateContactsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;
    let user_id = params.user_id;
    let contact_type = params.contact_type;
    let name = params.name;
    let email = params.email;
    let phone = params.phone;
    let customer_id2 = params.customer_id2;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/customer/{customer_id}/contacts", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id));
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
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = user_id {
        local_var_form_params.insert("user_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = contact_type {
        local_var_form_params.insert("contact_type", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = email {
        local_var_form_params.insert("email", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = phone {
        local_var_form_params.insert("phone", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = customer_id2 {
        local_var_form_params.insert("customer_id", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

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
        let local_var_entity: Option<CreateContactsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a contact.
pub async fn delete_contact(configuration: &mut configuration::Configuration, params: DeleteContactParams) -> Result<crate::models::InlineResponse200, Error<DeleteContactError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;
    let contact_id = params.contact_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/customer/{customer_id}/contacts/{contact_id}", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id), contact_id=crate::apis::urlencode(contact_id));
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteContactError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all contacts from a specified customer ID.
pub async fn list_contacts(configuration: &mut configuration::Configuration, params: ListContactsParams) -> Result<Vec<crate::models::SchemasContactResponse>, Error<ListContactsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/customer/{customer_id}/contacts", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id));
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
        let local_var_entity: Option<ListContactsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

