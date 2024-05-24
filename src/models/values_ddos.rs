/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ValuesDdos : The results of the query, optionally filtered and grouped over the requested timespan.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValuesDdos {
    /// For HTTP/2, the number of connections the limit-streams action was applied to. The limit-streams action caps the allowed number of concurrent streams in a connection.
    #[serde(rename = "ddos_action_limit_streams_connections", skip_serializing_if = "Option::is_none")]
    pub ddos_action_limit_streams_connections: Option<i32>,
    /// For HTTP/2, the number of requests made on a connection for which the limit-streams action was taken. The limit-streams action caps the allowed number of concurrent streams in a connection.
    #[serde(rename = "ddos_action_limit_streams_requests", skip_serializing_if = "Option::is_none")]
    pub ddos_action_limit_streams_requests: Option<i32>,
    /// The number of times the tarpit-accept action was taken. The tarpit-accept action adds a delay when accepting future connections.
    #[serde(rename = "ddos_action_tarpit_accept", skip_serializing_if = "Option::is_none")]
    pub ddos_action_tarpit_accept: Option<i32>,
    /// The number of times the tarpit action was taken. The tarpit action delays writing the response to the client.
    #[serde(rename = "ddos_action_tarpit", skip_serializing_if = "Option::is_none")]
    pub ddos_action_tarpit: Option<i32>,
    /// The number of times the close action was taken. The close action aborts the connection as soon as possible. The close action takes effect either right after accept, right after the client hello, or right after the response was sent.
    #[serde(rename = "ddos_action_close", skip_serializing_if = "Option::is_none")]
    pub ddos_action_close: Option<i32>,
    /// The number of times the blackhole action was taken. The blackhole action quietly closes a TCP connection without sending a reset. The blackhole action quietly closes a TCP connection without notifying its peer (all TCP state is dropped).
    #[serde(rename = "ddos_action_blackhole", skip_serializing_if = "Option::is_none")]
    pub ddos_action_blackhole: Option<i32>,
}

impl ValuesDdos {
    /// The results of the query, optionally filtered and grouped over the requested timespan.
    pub fn new() -> ValuesDdos {
        ValuesDdos {
            ddos_action_limit_streams_connections: None,
            ddos_action_limit_streams_requests: None,
            ddos_action_tarpit_accept: None,
            ddos_action_tarpit: None,
            ddos_action_close: None,
            ddos_action_blackhole: None,
        }
    }
}


