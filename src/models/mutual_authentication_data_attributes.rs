/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MutualAuthenticationDataAttributes {
    /// One or more certificates. Enter each individual certificate blob on a new line. Must be PEM-formatted. Required on create. You may optionally rotate the cert_bundle on update.
    #[serde(rename = "cert_bundle", skip_serializing_if = "Option::is_none")]
    pub cert_bundle: Option<String>,
    /// Determines whether Mutual TLS will fail closed (enforced) or fail open. A true value will require a successful Mutual TLS handshake for the connection to continue and will fail closed if unsuccessful. A false value will fail open and allow the connection to proceed. Optional. Defaults to true.
    #[serde(rename = "enforced", skip_serializing_if = "Option::is_none")]
    pub enforced: Option<bool>,
    /// A custom name for your mutual authentication. Optional. If name is not supplied we will auto-generate one.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl MutualAuthenticationDataAttributes {
    pub fn new() -> MutualAuthenticationDataAttributes {
        MutualAuthenticationDataAttributes {
            cert_bundle: None,
            enforced: None,
            name: None,
        }
    }
}


