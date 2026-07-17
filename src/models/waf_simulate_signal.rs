/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// WafSimulateSignal : A signal detected during WAF simulation. The `type`, `detector`, `detector_scope`, and `redaction` fields are always present. The `location`, `name`, and `value` fields are present only when applicable to the signal category.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafSimulateSignal {
    /// The type of signal detected (e.g., `SQLI`, `XSS`, `CMDEXE`, `TRAVERSAL`, `BACKDOOR`, `LOG4J-JNDI`, `BLOCKED`).
    #[serde(rename = "type")]
    pub _type: String,
    /// The detector engine that identified the signal (e.g., `SQLI`, `LIBINJECTIONV5`, `LIBINJECTIONJS`, or a rule ID).
    #[serde(rename = "detector")]
    pub detector: String,
    /// The scope of the detector that identified the signal. Derived from the signal type and detection type at simulation time. `system` — built-in WAF rule (e.g., `SQLI`, `XSS`). `workspace` — workspace-level custom rule or signal (e.g., `site.*` prefix). `account` — account-level custom signal (e.g., `corp.*` prefix). `unknown` — scope could not be determined (e.g., tags fetch failed or unrecognized type).
    #[serde(rename = "detector_scope")]
    pub detector_scope: DetectorScope,
    /// The redaction level applied to the detected value. Clients should handle unexpected string values gracefully, as new redaction types may be added.
    #[serde(rename = "redaction")]
    pub redaction: Redaction,
    /// Where in the request the signal was detected (e.g., `QUERYSTRING`, `POSTBODY`, `HEADER`, `HEADEROUT`, `POSTARG`). Present for detection signals; absent for custom and action signals.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// The parameter or header name that triggered detection. Present when the WAF engine identifies a specific parameter or header.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The matched payload value that triggered signal detection. For detection signals, contains the matched content. For `BLOCKED` signals, carries the WAF response code as a string. Absent for custom signals.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl WafSimulateSignal {
    /// A signal detected during WAF simulation. The `type`, `detector`, `detector_scope`, and `redaction` fields are always present. The `location`, `name`, and `value` fields are present only when applicable to the signal category.
    pub fn new(_type: String, detector: String, detector_scope: DetectorScope, redaction: Redaction) -> WafSimulateSignal {
        WafSimulateSignal {
            _type,
            detector,
            detector_scope,
            redaction,
            location: None,
            name: None,
            value: None,
        }
    }
}

/// The scope of the detector that identified the signal. Derived from the signal type and detection type at simulation time. `system` — built-in WAF rule (e.g., `SQLI`, `XSS`). `workspace` — workspace-level custom rule or signal (e.g., `site.*` prefix). `account` — account-level custom signal (e.g., `corp.*` prefix). `unknown` — scope could not be determined (e.g., tags fetch failed or unrecognized type).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DetectorScope {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "workspace")]
    Workspace,
    #[serde(rename = "account")]
    Account,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for DetectorScope {
    fn default() -> DetectorScope {
        Self::System
    }
}
/// The redaction level applied to the detected value. Clients should handle unexpected string values gracefully, as new redaction types may be added.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Redaction {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "param")]
    Param,
    #[serde(rename = "credit_card")]
    CreditCard,
    #[serde(rename = "ssn")]
    Ssn,
    #[serde(rename = "guid")]
    Guid,
    #[serde(rename = "iban")]
    Iban,
    #[serde(rename = "request_header")]
    RequestHeader,
    #[serde(rename = "response_header")]
    ResponseHeader,
    #[serde(rename = "custom_param")]
    CustomParam,
    #[serde(rename = "custom_request_header")]
    CustomRequestHeader,
    #[serde(rename = "custom_response_header")]
    CustomResponseHeader,
    #[serde(rename = "jsession_id")]
    JsessionId,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for Redaction {
    fn default() -> Redaction {
        Self::None
    }
}

