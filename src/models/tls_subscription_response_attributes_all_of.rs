/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TlsSubscriptionResponseAttributesAllOf {
    /// The current state of your subscription.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl TlsSubscriptionResponseAttributesAllOf {
    pub fn new() -> TlsSubscriptionResponseAttributesAllOf {
        TlsSubscriptionResponseAttributesAllOf {
            state: None,
        }
    }
}

/// The current state of your subscription.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "issued")]
    Issued,
    #[serde(rename = "renewing")]
    Renewing,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for State {
    fn default() -> State {
        Self::Pending
    }
}

