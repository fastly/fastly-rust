/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */


use reqwest;
use std::env;

// https://developer.fastly.com/reference/api/#rate-limiting
pub const DEFAULT_RATELIMIT: u64 = 1000;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    pub rate_limit_remaining: u64,
    pub rate_limit_reset: u64,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let api_key = match env::var("FASTLY_API_TOKEN") {
          Ok(key) => key,
          Err(_) => "".to_string(),
        };

        Configuration {
            base_path: "https://api.fastly.com".to_owned(),
            user_agent: Some("fastly-api/3.0.0/rust".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: Some(ApiKey {
              prefix: None,
              key: api_key,
            }),
            rate_limit_remaining: DEFAULT_RATELIMIT,
            rate_limit_reset: 0,
        }
    }
}
