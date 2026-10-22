# AutomationTokenCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the token. | 
**role** | Option<**String**> | The primary role on the token. For tokens with multiple roles, this returns the highest-priority role for backwards compatibility. | 
**roles** | Option<[**Vec&lt;crate::models::AutomationTokenRole&gt;**](AutomationTokenRole.md)> | All roles assigned to the token. Each role includes an `id` and `name`. | 
**services** | Option<**Vec<String>**> | The service IDs of the services the token has access to. Empty when `limit_services` is `false` (full access) or when the token has no service access. | 
**limit_services** | Option<**bool**> | When `true`, the token is restricted to only the services in the `services` array. When `false`, the token has access to all services. | 
**limit_workspaces** | Option<**bool**> | When `true`, the token is restricted to only the workspaces in the `workspaces` array. When `false`, the token has access to all workspaces. Only applies to roles that support workspace access. | 
**workspaces** | Option<**Vec<String>**> | The workspace IDs of the workspaces the token has access to. Returns an empty array for tokens without workspace access. | 
**scope** | Option<**String**> | A space-delimited list of authorization scope. | [default to Scope_Global]
**expires_at** | Option<**String**> | A UTC timestamp of when the token expires. | 
**id** | Option<[**crate::models::ReadOnlyId**](ReadOnlyId.md)> |  | 
**user_id** | Option<[**crate::models::ReadOnlyUserId**](ReadOnlyUserId.md)> |  | 
**customer_id** | Option<[**crate::models::ReadOnlyCustomerId**](ReadOnlyCustomerId.md)> |  | 
**created_at** | Option<**String**> | A UTC timestamp of when the token was created.  | [readonly]
**access_token** | Option<**String**> |  | [readonly]
**tls_access** | Option<**bool**> | Indicates whether TLS access is enabled for the token. | 
**last_used_at** | Option<**String**> | A UTC timestamp of when the token was last used. | [readonly]
**user_agent** | Option<**String**> | The User-Agent header of the client that last used the token. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


