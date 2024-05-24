/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HeaderResponseAdditional {
    /// Don't add the header if it is added already. Only applies to 'set' action. Numerical value (\"0\" = false, \"1\" = true)
    #[serde(rename = "ignore_if_set", skip_serializing_if = "Option::is_none")]
    pub ignore_if_set: Option<String>,
    /// Priority determines execution order. Lower numbers execute first.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}

impl HeaderResponseAdditional {
    pub fn new() -> HeaderResponseAdditional {
        HeaderResponseAdditional {
            ignore_if_set: None,
            priority: None,
        }
    }
}


