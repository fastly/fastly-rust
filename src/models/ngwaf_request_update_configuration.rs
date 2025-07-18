/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NgwafRequestUpdateConfiguration {
    /// The workspace to link.
    #[serde(rename = "workspace_id", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// The percentage of traffic to inspect.
    #[serde(rename = "traffic_ramp", skip_serializing_if = "Option::is_none")]
    pub traffic_ramp: Option<String>,
}

impl NgwafRequestUpdateConfiguration {
    pub fn new() -> NgwafRequestUpdateConfiguration {
        NgwafRequestUpdateConfiguration {
            workspace_id: None,
            traffic_ramp: None,
        }
    }
}


