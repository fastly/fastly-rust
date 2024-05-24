/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_user`]
#[derive(Clone, Debug, Default)]
pub struct CreateUserParams {
    pub login: Option<String>,
    /// The real life name of the user.
    pub name: Option<String>,
    /// Indicates that the user has limited access to the customer's services.
    pub limit_services: Option<bool>,
    /// Indicates whether the is account is locked for editing or not.
    pub locked: Option<bool>,
    /// Indicates if a new password is required at next login.
    pub require_new_password: Option<bool>,
    pub role: Option<crate::models::RoleUser>,
    /// Indicates if 2FA is enabled on the user.
    pub two_factor_auth_enabled: Option<bool>,
    /// Indicates if 2FA is required by the user's customer account.
    pub two_factor_setup_required: Option<bool>
}

/// struct for passing parameters to the method [`delete_user`]
#[derive(Clone, Debug, Default)]
pub struct DeleteUserParams {
    /// Alphanumeric string identifying the user.
    pub user_id: String
}

/// struct for passing parameters to the method [`get_user`]
#[derive(Clone, Debug, Default)]
pub struct GetUserParams {
    /// Alphanumeric string identifying the user.
    pub user_id: String
}

/// struct for passing parameters to the method [`request_password_reset`]
#[derive(Clone, Debug, Default)]
pub struct RequestPasswordResetParams {
    /// The login associated with the user (typically, an email address).
    pub user_login: String
}

/// struct for passing parameters to the method [`update_user`]
#[derive(Clone, Debug, Default)]
pub struct UpdateUserParams {
    /// Alphanumeric string identifying the user.
    pub user_id: String,
    pub login: Option<String>,
    /// The real life name of the user.
    pub name: Option<String>,
    /// Indicates that the user has limited access to the customer's services.
    pub limit_services: Option<bool>,
    /// Indicates whether the is account is locked for editing or not.
    pub locked: Option<bool>,
    /// Indicates if a new password is required at next login.
    pub require_new_password: Option<bool>,
    pub role: Option<crate::models::RoleUser>,
    /// Indicates if 2FA is enabled on the user.
    pub two_factor_auth_enabled: Option<bool>,
    /// Indicates if 2FA is required by the user's customer account.
    pub two_factor_setup_required: Option<bool>
}

/// struct for passing parameters to the method [`update_user_password`]
#[derive(Clone, Debug, Default)]
pub struct UpdateUserPasswordParams {
    /// The user's current password.
    pub old_password: Option<String>,
    /// The user's new password.
    pub new_password: Option<String>
}


/// struct for typed errors of method [`create_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUserError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_current_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCurrentUserError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`request_password_reset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestPasswordResetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user_password`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserPasswordError {
    UnknownValue(serde_json::Value),
}


/// Create a user.
pub async fn create_user(configuration: &mut configuration::Configuration, params: CreateUserParams) -> Result<crate::models::UserResponse, Error<CreateUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let login = params.login;
    let name = params.name;
    let limit_services = params.limit_services;
    let locked = params.locked;
    let require_new_password = params.require_new_password;
    let role = params.role;
    let two_factor_auth_enabled = params.two_factor_auth_enabled;
    let two_factor_setup_required = params.two_factor_setup_required;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user", local_var_configuration.base_path);
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
    if let Some(local_var_param_value) = login {
        local_var_form_params.insert("login", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = limit_services {
        local_var_form_params.insert("limit_services", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = locked {
        local_var_form_params.insert("locked", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = require_new_password {
        local_var_form_params.insert("require_new_password", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = role {
        local_var_form_params.insert("role", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = two_factor_auth_enabled {
        local_var_form_params.insert("two_factor_auth_enabled", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = two_factor_setup_required {
        local_var_form_params.insert("two_factor_setup_required", local_var_param_value.to_string());
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
        let local_var_entity: Option<CreateUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a user.
pub async fn delete_user(configuration: &mut configuration::Configuration, params: DeleteUserParams) -> Result<crate::models::InlineResponse200, Error<DeleteUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let user_id = params.user_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user/{user_id}", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id));
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
        let local_var_entity: Option<DeleteUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the logged in user.
pub async fn get_current_user(configuration: &mut configuration::Configuration) -> Result<crate::models::UserResponse, Error<GetCurrentUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/current_user", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetCurrentUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a specific user.
pub async fn get_user(configuration: &mut configuration::Configuration, params: GetUserParams) -> Result<crate::models::UserResponse, Error<GetUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let user_id = params.user_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user/{user_id}", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id));
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
        let local_var_entity: Option<GetUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Requests a password reset for the specified user.
pub async fn request_password_reset(configuration: &mut configuration::Configuration, params: RequestPasswordResetParams) -> Result<crate::models::InlineResponse200, Error<RequestPasswordResetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let user_login = params.user_login;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user/{user_login}/password/request_reset", local_var_configuration.base_path, user_login=crate::apis::urlencode(user_login));
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
        let local_var_entity: Option<RequestPasswordResetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a user. Only users with the role of `superuser` can make changes to other users on the account. Non-superusers may use this endpoint to make changes to their own account. Two-factor attributes are not editable via this endpoint.
pub async fn update_user(configuration: &mut configuration::Configuration, params: UpdateUserParams) -> Result<crate::models::UserResponse, Error<UpdateUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let user_id = params.user_id;
    let login = params.login;
    let name = params.name;
    let limit_services = params.limit_services;
    let locked = params.locked;
    let require_new_password = params.require_new_password;
    let role = params.role;
    let two_factor_auth_enabled = params.two_factor_auth_enabled;
    let two_factor_setup_required = params.two_factor_setup_required;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user/{user_id}", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id));
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
    if let Some(local_var_param_value) = login {
        local_var_form_params.insert("login", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = limit_services {
        local_var_form_params.insert("limit_services", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = locked {
        local_var_form_params.insert("locked", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = require_new_password {
        local_var_form_params.insert("require_new_password", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = role {
        local_var_form_params.insert("role", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = two_factor_auth_enabled {
        local_var_form_params.insert("two_factor_auth_enabled", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = two_factor_setup_required {
        local_var_form_params.insert("two_factor_setup_required", local_var_param_value.to_string());
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
        let local_var_entity: Option<UpdateUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the user's password to a new one.
pub async fn update_user_password(configuration: &mut configuration::Configuration, params: UpdateUserPasswordParams) -> Result<crate::models::UserResponse, Error<UpdateUserPasswordError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let old_password = params.old_password;
    let new_password = params.new_password;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/current_user/password", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = old_password {
        local_var_form_params.insert("old_password", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = new_password {
        local_var_form_params.insert("new_password", local_var_param_value.to_string());
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
        let local_var_entity: Option<UpdateUserPasswordError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

