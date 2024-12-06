/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SetConfiguration {
    /// The new workspace_id. Required in the `PUT` request body when `product_id` is `ngwaf`. Optional in the `PATCH` request body for `ngwaf`.
    #[serde(rename = "workspace_id", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// The new traffic ramp. Optional in the `PATCH` request body for `ngwaf`.
    #[serde(rename = "traffic_ramp", skip_serializing_if = "Option::is_none")]
    pub traffic_ramp: Option<String>,
    /// The new mode to run the product in. One of `block`, `log`, or `off`. Optional in the `PATCH` request body for `ddos_protection`.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

impl SetConfiguration {
    pub fn new() -> SetConfiguration {
        SetConfiguration {
            workspace_id: None,
            traffic_ramp: None,
            mode: None,
        }
    }
}


