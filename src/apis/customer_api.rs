/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`delete_customer`]
#[derive(Clone, Debug, Default)]
pub struct DeleteCustomerParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String
}

/// struct for passing parameters to the method [`get_customer`]
#[derive(Clone, Debug, Default)]
pub struct GetCustomerParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String
}

/// struct for passing parameters to the method [`list_users`]
#[derive(Clone, Debug, Default)]
pub struct ListUsersParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String
}

/// struct for passing parameters to the method [`update_customer`]
#[derive(Clone, Debug, Default)]
pub struct UpdateCustomerParams {
    /// Alphanumeric string identifying the customer.
    pub customer_id: String,
    /// The alphanumeric string representing the primary billing contact.
    pub billing_contact_id: Option<String>,
    /// Customer's current network revenue type.
    pub billing_network_type: Option<String>,
    /// Used for adding purchased orders to customer's account.
    pub billing_ref: Option<String>,
    /// Whether this customer can view or edit wordpress.
    pub can_configure_wordpress: Option<bool>,
    /// Whether this customer can reset passwords.
    pub can_reset_passwords: Option<bool>,
    /// Whether this customer can upload VCL.
    pub can_upload_vcl: Option<bool>,
    /// Specifies whether 2FA is forced or not forced on the customer account. Logs out non-2FA users once 2FA is force enabled.
    pub force_2fa: Option<bool>,
    /// Specifies whether SSO is forced or not forced on the customer account.
    pub force_sso: Option<bool>,
    /// Specifies whether the account has access or does not have access to the account panel.
    pub has_account_panel: Option<bool>,
    /// Specifies whether the account has access or does not have access to the improved events.
    pub has_improved_events: Option<bool>,
    /// Whether this customer can view or edit the SSL config.
    pub has_improved_ssl_config: Option<bool>,
    /// Specifies whether the account has enabled or not enabled openstack logging.
    pub has_openstack_logging: Option<bool>,
    /// Specifies whether the account can edit PCI for a service.
    pub has_pci: Option<bool>,
    /// Specifies whether PCI passwords are required for the account.
    pub has_pci_passwords: Option<bool>,
    /// The range of IP addresses authorized to access the customer account.
    pub ip_whitelist: Option<String>,
    /// The alphanumeric string identifying the account's legal contact.
    pub legal_contact_id: Option<String>,
    /// The name of the customer, generally the company name.
    pub name: Option<String>,
    /// The alphanumeric string identifying the account owner.
    pub owner_id: Option<String>,
    /// The phone number associated with the account.
    pub phone_number: Option<String>,
    /// The postal address associated with the account.
    pub postal_address: Option<String>,
    /// The pricing plan this customer is under.
    pub pricing_plan: Option<String>,
    /// The alphanumeric string identifying the pricing plan.
    pub pricing_plan_id: Option<String>,
    /// The alphanumeric string identifying the account's security contact.
    pub security_contact_id: Option<String>,
    /// The alphanumeric string identifying the account's technical contact.
    pub technical_contact_id: Option<String>
}


/// struct for typed errors of method [`delete_customer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCustomerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_customer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCustomerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_logged_in_customer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLoggedInCustomerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUsersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_customer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCustomerError {
    UnknownValue(serde_json::Value),
}


/// Delete a customer.
pub async fn delete_customer(configuration: &mut configuration::Configuration, params: DeleteCustomerParams) -> Result<crate::models::InlineResponse200, Error<DeleteCustomerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/customer/{customer_id}", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id));
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
        let local_var_entity: Option<DeleteCustomerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a specific customer.
pub async fn get_customer(configuration: &mut configuration::Configuration, params: GetCustomerParams) -> Result<crate::models::CustomerResponse, Error<GetCustomerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/customer/{customer_id}", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id));
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
        let local_var_entity: Option<GetCustomerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the logged in customer.
pub async fn get_logged_in_customer(configuration: &mut configuration::Configuration) -> Result<crate::models::CustomerResponse, Error<GetLoggedInCustomerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/current_customer", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetLoggedInCustomerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all users from a specified customer id.
pub async fn list_users(configuration: &mut configuration::Configuration, params: ListUsersParams) -> Result<Vec<crate::models::SchemasUserResponse>, Error<ListUsersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/customer/{customer_id}/users", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id));
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
        let local_var_entity: Option<ListUsersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a customer.
pub async fn update_customer(configuration: &mut configuration::Configuration, params: UpdateCustomerParams) -> Result<crate::models::CustomerResponse, Error<UpdateCustomerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let customer_id = params.customer_id;
    let billing_contact_id = params.billing_contact_id;
    let billing_network_type = params.billing_network_type;
    let billing_ref = params.billing_ref;
    let can_configure_wordpress = params.can_configure_wordpress;
    let can_reset_passwords = params.can_reset_passwords;
    let can_upload_vcl = params.can_upload_vcl;
    let force_2fa = params.force_2fa;
    let force_sso = params.force_sso;
    let has_account_panel = params.has_account_panel;
    let has_improved_events = params.has_improved_events;
    let has_improved_ssl_config = params.has_improved_ssl_config;
    let has_openstack_logging = params.has_openstack_logging;
    let has_pci = params.has_pci;
    let has_pci_passwords = params.has_pci_passwords;
    let ip_whitelist = params.ip_whitelist;
    let legal_contact_id = params.legal_contact_id;
    let name = params.name;
    let owner_id = params.owner_id;
    let phone_number = params.phone_number;
    let postal_address = params.postal_address;
    let pricing_plan = params.pricing_plan;
    let pricing_plan_id = params.pricing_plan_id;
    let security_contact_id = params.security_contact_id;
    let technical_contact_id = params.technical_contact_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/customer/{customer_id}", local_var_configuration.base_path, customer_id=crate::apis::urlencode(customer_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
    if let Some(local_var_param_value) = billing_contact_id {
        local_var_form_params.insert("billing_contact_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = billing_network_type {
        local_var_form_params.insert("billing_network_type", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = billing_ref {
        local_var_form_params.insert("billing_ref", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = can_configure_wordpress {
        local_var_form_params.insert("can_configure_wordpress", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = can_reset_passwords {
        local_var_form_params.insert("can_reset_passwords", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = can_upload_vcl {
        local_var_form_params.insert("can_upload_vcl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = force_2fa {
        local_var_form_params.insert("force_2fa", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = force_sso {
        local_var_form_params.insert("force_sso", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = has_account_panel {
        local_var_form_params.insert("has_account_panel", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = has_improved_events {
        local_var_form_params.insert("has_improved_events", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = has_improved_ssl_config {
        local_var_form_params.insert("has_improved_ssl_config", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = has_openstack_logging {
        local_var_form_params.insert("has_openstack_logging", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = has_pci {
        local_var_form_params.insert("has_pci", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = has_pci_passwords {
        local_var_form_params.insert("has_pci_passwords", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ip_whitelist {
        local_var_form_params.insert("ip_whitelist", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = legal_contact_id {
        local_var_form_params.insert("legal_contact_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = owner_id {
        local_var_form_params.insert("owner_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = phone_number {
        local_var_form_params.insert("phone_number", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = postal_address {
        local_var_form_params.insert("postal_address", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = pricing_plan {
        local_var_form_params.insert("pricing_plan", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = pricing_plan_id {
        local_var_form_params.insert("pricing_plan_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = security_contact_id {
        local_var_form_params.insert("security_contact_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = technical_contact_id {
        local_var_form_params.insert("technical_contact_id", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "PUT" != "GET" && "PUT" != "HEAD" {
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
        let local_var_entity: Option<UpdateCustomerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

