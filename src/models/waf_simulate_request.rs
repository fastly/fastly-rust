/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// WafSimulateRequest : Request body for simulating a WAF request. The total request body must not exceed 200 KB.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafSimulateRequest {
    /// The raw HTTP request in wire format to simulate through the WAF. Must include the request line, headers, and optionally a body, separated by CRLF sequences.
    #[serde(rename = "request")]
    pub request: String,
    /// The raw HTTP response in wire format. The WAF engine inspects response headers during its PostRequest phase and may generate signals from them. When omitted, a default response of `HTTP/1.1 200 OK\\r\\n\\r\\n` is used.
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
}

impl WafSimulateRequest {
    /// Request body for simulating a WAF request. The total request body must not exceed 200 KB.
    pub fn new(request: String) -> WafSimulateRequest {
        WafSimulateRequest {
            request,
            response: None,
        }
    }
}


