use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod acl_api;
pub mod acl_entry_api;
pub mod apex_redirect_api;
pub mod automation_tokens_api;
pub mod backend_api;
pub mod billing_api;
pub mod billing_address_api;
pub mod cache_settings_api;
pub mod condition_api;
pub mod config_store_api;
pub mod config_store_item_api;
pub mod contact_api;
pub mod content_api;
pub mod customer_api;
pub mod dictionary_api;
pub mod dictionary_info_api;
pub mod dictionary_item_api;
pub mod diff_api;
pub mod director_api;
pub mod director_backend_api;
pub mod domain_api;
pub mod domain_inspector_historical_api;
pub mod domain_inspector_realtime_api;
pub mod domain_ownerships_api;
pub mod enabled_products_api;
pub mod events_api;
pub mod gzip_api;
pub mod header_api;
pub mod healthcheck_api;
pub mod http3_api;
pub mod iam_permissions_api;
pub mod iam_roles_api;
pub mod iam_service_groups_api;
pub mod iam_user_groups_api;
pub mod invitations_api;
pub mod kv_store_api;
pub mod kv_store_item_api;
pub mod legacy_waf_configuration_sets_api;
pub mod legacy_waf_firewall_api;
pub mod legacy_waf_owasp_api;
pub mod legacy_waf_rule_api;
pub mod legacy_waf_rule_status_api;
pub mod legacy_waf_ruleset_api;
pub mod legacy_waf_tag_api;
pub mod legacy_waf_update_status_api;
pub mod logging_azureblob_api;
pub mod logging_bigquery_api;
pub mod logging_cloudfiles_api;
pub mod logging_datadog_api;
pub mod logging_digitalocean_api;
pub mod logging_elasticsearch_api;
pub mod logging_ftp_api;
pub mod logging_gcs_api;
pub mod logging_heroku_api;
pub mod logging_honeycomb_api;
pub mod logging_https_api;
pub mod logging_kafka_api;
pub mod logging_kinesis_api;
pub mod logging_logentries_api;
pub mod logging_loggly_api;
pub mod logging_logshuttle_api;
pub mod logging_newrelic_api;
pub mod logging_openstack_api;
pub mod logging_papertrail_api;
pub mod logging_pubsub_api;
pub mod logging_s3_api;
pub mod logging_scalyr_api;
pub mod logging_sftp_api;
pub mod logging_splunk_api;
pub mod logging_sumologic_api;
pub mod logging_syslog_api;
pub mod mutual_authentication_api;
pub mod origin_inspector_historical_api;
pub mod origin_inspector_realtime_api;
pub mod package_api;
pub mod pool_api;
pub mod pop_api;
pub mod public_ip_list_api;
pub mod publish_api;
pub mod purge_api;
pub mod rate_limiter_api;
pub mod realtime_api;
pub mod request_settings_api;
pub mod resource_api;
pub mod response_object_api;
pub mod secret_store_api;
pub mod secret_store_item_api;
pub mod server_api;
pub mod service_api;
pub mod service_authorizations_api;
pub mod settings_api;
pub mod snippet_api;
pub mod star_api;
pub mod stats_api;
pub mod sudo_api;
pub mod tls_activations_api;
pub mod tls_bulk_certificates_api;
pub mod tls_certificates_api;
pub mod tls_configurations_api;
pub mod tls_domains_api;
pub mod tls_private_keys_api;
pub mod tls_subscriptions_api;
pub mod tokens_api;
pub mod user_api;
pub mod vcl_api;
pub mod vcl_diff_api;
pub mod version_api;
pub mod waf_active_rules_api;
pub mod waf_exclusions_api;
pub mod waf_firewall_versions_api;
pub mod waf_firewalls_api;
pub mod waf_rule_revisions_api;
pub mod waf_rules_api;
pub mod waf_tags_api;

pub mod configuration;
