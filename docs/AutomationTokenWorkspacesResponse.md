# AutomationTokenWorkspacesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**Vec&lt;crate::models::AutomationTokenWorkspace&gt;**](AutomationTokenWorkspace.md) | Workspaces the token can access. Empty if the token has no workspace-specific permissions or if `limit_workspaces` is `false` (unrestricted access). | 
**meta** | [**crate::models::AutomationTokenWorkspacesMeta**](AutomationTokenWorkspacesMeta.md) |  | 
**limit_workspaces** | **bool** | Indicates whether workspace access is restricted: - `true`: Token can only access workspaces listed in `data` - `false`: Token has unrestricted access to all workspaces  | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


