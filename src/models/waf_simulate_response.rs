/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// WafSimulateResponse : Response from the WAF simulation containing the WAF response code and detected signals.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafSimulateResponse {
    /// The HTTP status code the WAF would return for the simulated request (e.g., `200` for allowed, `406` for blocked).
    #[serde(rename = "waf_response")]
    pub waf_response: i32,
    /// List of signals detected by the WAF during simulation. Empty array when no signals are detected.
    #[serde(rename = "signals")]
    pub signals: Vec<crate::models::WafSimulateSignal>,
}

impl WafSimulateResponse {
    /// Response from the WAF simulation containing the WAF response code and detected signals.
    pub fn new(waf_response: i32, signals: Vec<crate::models::WafSimulateSignal>) -> WafSimulateResponse {
        WafSimulateResponse {
            waf_response,
            signals,
        }
    }
}


