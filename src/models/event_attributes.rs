/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EventAttributes {
    /// Indicates if event was performed by Fastly.
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Box<String>>,
    /// Description of the event.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Type of event. Can be used with `filter[event_type]`
    #[serde(rename = "event_type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,
    /// IP addresses that the event was requested from.
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Hash of key value pairs of additional information.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<String>>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Box<String>>,
    #[serde(rename = "token_id", skip_serializing_if = "Option::is_none")]
    pub token_id: Option<Box<String>>,
}

impl EventAttributes {
    pub fn new() -> EventAttributes {
        EventAttributes {
            admin: None,
            created_at: None,
            customer_id: None,
            description: None,
            event_type: None,
            ip: None,
            metadata: None,
            service_id: None,
            user_id: None,
            token_id: None,
        }
    }
}

/// Type of event. Can be used with `filter[event_type]`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "api_key.create")]
    ApiKeyCreate,
    #[serde(rename = "acl.create")]
    AclCreate,
    #[serde(rename = "acl.delete")]
    AclDelete,
    #[serde(rename = "acl.update")]
    AclUpdate,
    #[serde(rename = "address.create")]
    AddressCreate,
    #[serde(rename = "address.delete")]
    AddressDelete,
    #[serde(rename = "address.update")]
    AddressUpdate,
    #[serde(rename = "backend.create")]
    BackendCreate,
    #[serde(rename = "backend.delete")]
    BackendDelete,
    #[serde(rename = "backend.update")]
    BackendUpdate,
    #[serde(rename = "billing.contact_update")]
    BillingContactUpdate,
    #[serde(rename = "cache_settings.create")]
    CacheSettingsCreate,
    #[serde(rename = "cache_settings.delete")]
    CacheSettingsDelete,
    #[serde(rename = "cache_settings.update")]
    CacheSettingsUpdate,
    #[serde(rename = "customer.create")]
    CustomerCreate,
    #[serde(rename = "customer.pricing")]
    CustomerPricing,
    #[serde(rename = "customer.update")]
    CustomerUpdate,
    #[serde(rename = "customer_feature.create")]
    CustomerFeatureCreate,
    #[serde(rename = "customer_feature.delete")]
    CustomerFeatureDelete,
    #[serde(rename = "director.create")]
    DirectorCreate,
    #[serde(rename = "director.delete")]
    DirectorDelete,
    #[serde(rename = "director.update")]
    DirectorUpdate,
    #[serde(rename = "director_backend.create")]
    DirectorBackendCreate,
    #[serde(rename = "director_backend.delete")]
    DirectorBackendDelete,
    #[serde(rename = "domain.create")]
    DomainCreate,
    #[serde(rename = "domain.delete")]
    DomainDelete,
    #[serde(rename = "domain.update")]
    DomainUpdate,
    #[serde(rename = "gzip.create")]
    GzipCreate,
    #[serde(rename = "gzip.delete")]
    GzipDelete,
    #[serde(rename = "gzip.update")]
    GzipUpdate,
    #[serde(rename = "header.create")]
    HeaderCreate,
    #[serde(rename = "header.delete")]
    HeaderDelete,
    #[serde(rename = "header.update")]
    HeaderUpdate,
    #[serde(rename = "healthcheck.create")]
    HealthcheckCreate,
    #[serde(rename = "healthcheck.delete")]
    HealthcheckDelete,
    #[serde(rename = "healthcheck.update")]
    HealthcheckUpdate,
    #[serde(rename = "invitation.accept")]
    InvitationAccept,
    #[serde(rename = "invitation.sent")]
    InvitationSent,
    #[serde(rename = "invoice.failed_payment")]
    InvoiceFailedPayment,
    #[serde(rename = "invoice.payment")]
    InvoicePayment,
    #[serde(rename = "io_settings.create")]
    IoSettingsCreate,
    #[serde(rename = "io_settings.delete")]
    IoSettingsDelete,
    #[serde(rename = "io_settings.update")]
    IoSettingsUpdate,
    #[serde(rename = "logging.create")]
    LoggingCreate,
    #[serde(rename = "logging.delete")]
    LoggingDelete,
    #[serde(rename = "logging.update")]
    LoggingUpdate,
    #[serde(rename = "pool.create")]
    PoolCreate,
    #[serde(rename = "pool.delete")]
    PoolDelete,
    #[serde(rename = "pool.update")]
    PoolUpdate,
    #[serde(rename = "request_settings.create")]
    RequestSettingsCreate,
    #[serde(rename = "request_settings.delete")]
    RequestSettingsDelete,
    #[serde(rename = "request_settings.update")]
    RequestSettingsUpdate,
    #[serde(rename = "response_object.create")]
    ResponseObjectCreate,
    #[serde(rename = "response_object.delete")]
    ResponseObjectDelete,
    #[serde(rename = "response_object.update")]
    ResponseObjectUpdate,
    #[serde(rename = "rule_status.update")]
    RuleStatusUpdate,
    #[serde(rename = "rule_status.upsert")]
    RuleStatusUpsert,
    #[serde(rename = "server.create")]
    ServerCreate,
    #[serde(rename = "server.delete")]
    ServerDelete,
    #[serde(rename = "server.update")]
    ServerUpdate,
    #[serde(rename = "service.create")]
    ServiceCreate,
    #[serde(rename = "service.delete")]
    ServiceDelete,
    #[serde(rename = "service.move")]
    ServiceMove,
    #[serde(rename = "service.move_destination")]
    ServiceMoveDestination,
    #[serde(rename = "service.move_source")]
    ServiceMoveSource,
    #[serde(rename = "service.purge_all")]
    ServicePurgeAll,
    #[serde(rename = "service.update")]
    ServiceUpdate,
    #[serde(rename = "service_authorization.create")]
    ServiceAuthorizationCreate,
    #[serde(rename = "service_authorization.delete")]
    ServiceAuthorizationDelete,
    #[serde(rename = "service_authorization.update")]
    ServiceAuthorizationUpdate,
    #[serde(rename = "tls.bulk_certificate.create")]
    TlsBulkCertificateCreate,
    #[serde(rename = "tls.bulk_certificate.delete")]
    TlsBulkCertificateDelete,
    #[serde(rename = "tls.bulk_certificate.update")]
    TlsBulkCertificateUpdate,
    #[serde(rename = "tls.certificate.create")]
    TlsCertificateCreate,
    #[serde(rename = "tls.certificate.expiration_email")]
    TlsCertificateExpirationEmail,
    #[serde(rename = "tls.certificate.update")]
    TlsCertificateUpdate,
    #[serde(rename = "tls.certificate.delete")]
    TlsCertificateDelete,
    #[serde(rename = "tls.configuration.update")]
    TlsConfigurationUpdate,
    #[serde(rename = "tls.private_key.create")]
    TlsPrivateKeyCreate,
    #[serde(rename = "tls.private_key.delete")]
    TlsPrivateKeyDelete,
    #[serde(rename = "tls.activation.enable")]
    TlsActivationEnable,
    #[serde(rename = "tls.activation.update")]
    TlsActivationUpdate,
    #[serde(rename = "tls.activation.disable")]
    TlsActivationDisable,
    #[serde(rename = "tls.globalsign.domain.create")]
    TlsGlobalsignDomainCreate,
    #[serde(rename = "tls.globalsign.domain.verify")]
    TlsGlobalsignDomainVerify,
    #[serde(rename = "tls.globalsign.domain.delete")]
    TlsGlobalsignDomainDelete,
    #[serde(rename = "tls.subscription.create")]
    TlsSubscriptionCreate,
    #[serde(rename = "tls.subscription.delete")]
    TlsSubscriptionDelete,
    #[serde(rename = "tls.subscription.dns_check_email")]
    TlsSubscriptionDnsCheckEmail,
    #[serde(rename = "token.create")]
    TokenCreate,
    #[serde(rename = "token.destroy")]
    TokenDestroy,
    #[serde(rename = "two_factor_auth.disable")]
    TwoFactorAuthDisable,
    #[serde(rename = "two_factor_auth.enable")]
    TwoFactorAuthEnable,
    #[serde(rename = "user.create")]
    UserCreate,
    #[serde(rename = "user.destroy")]
    UserDestroy,
    #[serde(rename = "user.lock")]
    UserLock,
    #[serde(rename = "user.login")]
    UserLogin,
    #[serde(rename = "user.login_failure")]
    UserLoginFailure,
    #[serde(rename = "user.logout")]
    UserLogout,
    #[serde(rename = "user.password_update")]
    UserPasswordUpdate,
    #[serde(rename = "user.unlock")]
    UserUnlock,
    #[serde(rename = "user.update")]
    UserUpdate,
    #[serde(rename = "vcl.create")]
    VclCreate,
    #[serde(rename = "vcl.delete")]
    VclDelete,
    #[serde(rename = "vcl.update")]
    VclUpdate,
    #[serde(rename = "version.activate")]
    VersionActivate,
    #[serde(rename = "version.clone")]
    VersionClone,
    #[serde(rename = "version.copy")]
    VersionCopy,
    #[serde(rename = "version.copy_destination")]
    VersionCopyDestination,
    #[serde(rename = "version.copy_source")]
    VersionCopySource,
    #[serde(rename = "version.create")]
    VersionCreate,
    #[serde(rename = "version.deactivate")]
    VersionDeactivate,
    #[serde(rename = "version.lock")]
    VersionLock,
    #[serde(rename = "version.update")]
    VersionUpdate,
    #[serde(rename = "waf.configuration_set_update")]
    WafConfigurationSetUpdate,
    #[serde(rename = "waf.create")]
    WafCreate,
    #[serde(rename = "waf.delete")]
    WafDelete,
    #[serde(rename = "waf.update")]
    WafUpdate,
    #[serde(rename = "waf.enable")]
    WafEnable,
    #[serde(rename = "waf.disable")]
    WafDisable,
    #[serde(rename = "waf.owasp.create")]
    WafOwaspCreate,
    #[serde(rename = "waf.owasp.update")]
    WafOwaspUpdate,
    #[serde(rename = "waf.ruleset.deploy")]
    WafRulesetDeploy,
    #[serde(rename = "waf.ruleset.deploy_failure")]
    WafRulesetDeployFailure,
    #[serde(rename = "wordpress.create")]
    WordpressCreate,
    #[serde(rename = "wordpress.delete")]
    WordpressDelete,
    #[serde(rename = "wordpress.update")]
    WordpressUpdate,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::ApiKeyCreate
    }
}

