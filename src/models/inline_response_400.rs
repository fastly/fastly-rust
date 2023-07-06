/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse400 {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
}

impl InlineResponse400 {
    pub fn new() -> InlineResponse400 {
        InlineResponse400 {
            code: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "invalid_grant")]
    InvalidGrant,
    #[serde(rename = "invalid_request")]
    InvalidRequest,
    #[serde(rename = "invalid_scope")]
    InvalidScope,
    #[serde(rename = "account_locked")]
    AccountLocked,
    #[serde(rename = "2fa.verify")]
    MfaVerify,
}

impl Default for Code {
    fn default() -> Code {
        Self::InvalidGrant
    }
}

