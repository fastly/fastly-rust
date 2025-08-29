/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// TopWorkspace : This object, found within the `top_workspaces` array, contains the workspace information and count for the requested signal.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TopWorkspace {
    /// ID of the workspace.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the workspace.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Count of attacks on this workspace for the specific attack type.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

impl TopWorkspace {
    /// This object, found within the `top_workspaces` array, contains the workspace information and count for the requested signal.
    pub fn new() -> TopWorkspace {
        TopWorkspace {
            id: None,
            name: None,
            count: None,
        }
    }
}


