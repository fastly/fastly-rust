/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AutomationTokenWorkspacesResponse {
    /// Workspaces the token can access. Empty if the token has no workspace-specific permissions or if `limit_workspaces` is `false` (unrestricted access).
    #[serde(rename = "data")]
    pub data: Vec<crate::models::AutomationTokenWorkspace>,
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::AutomationTokenWorkspacesMeta>,
    /// Indicates whether workspace access is restricted: - `true`: Token can only access workspaces listed in `data` - `false`: Token has unrestricted access to all workspaces 
    #[serde(rename = "limit_workspaces")]
    pub limit_workspaces: bool,
}

impl AutomationTokenWorkspacesResponse {
    pub fn new(data: Vec<crate::models::AutomationTokenWorkspace>, meta: crate::models::AutomationTokenWorkspacesMeta, limit_workspaces: bool) -> AutomationTokenWorkspacesResponse {
        AutomationTokenWorkspacesResponse {
            data,
            meta: Box::new(meta),
            limit_workspaces,
        }
    }
}


