/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SetWorkspaceId {
    /// The workspace to link with the Next-Gen WAF product. Note this body is only supported and required when `product_id` is `ngwaf`
    #[serde(rename = "workspace_id", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl SetWorkspaceId {
    pub fn new() -> SetWorkspaceId {
        SetWorkspaceId {
            workspace_id: None,
        }
    }
}


