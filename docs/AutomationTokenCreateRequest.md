# AutomationTokenCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the token. | 
**role** | Option<**String**> | The role for the token. The `roles` field is the preferred way to assign roles. This field is supported for backwards compatibility with single-role tokens. | 
**roles** | Option<**Vec<String>**> | Array of role IDs to assign to the token. Use this field for tokens with multiple roles. When provided, this takes precedence over the `role` field. To get available role IDs, use [GET /roles](https://www.fastly.com/documentation/reference/api/account/roles/). | 
**services** | Option<**Vec<String>**> | List of service IDs the token can access. Only used when `limit_services` is `true`. | 
**limit_services** | Option<**bool**> | When `true`, restricts the token to only the services specified in the `services` array. When `false` (default), the token has access to all services. | [default to false]
**limit_workspaces** | Option<**bool**> | When `true` (default), restricts the token to only the workspaces specified in the `workspaces` array. When `false`, the token has access to all workspaces. Only applies to roles that support workspace access (such as Next-Gen WAF roles). For roles without workspace support, this field has no effect. | [default to true]
**workspaces** | Option<**Vec<String>**> | List of workspace IDs the token can access. Only used when `limit_workspaces` is `true` and the token has roles that support workspace access. | 
**scope** | Option<**String**> |  | [default to Scope_Global]
**expires_at** | Option<**String**> | A UTC timestamp of when the token will expire. | 
**tls_access** | Option<**bool**> | Indicates whether TLS access is enabled for the token. Only applicable when using the single-role `role` field. For tokens with multiple roles, assign TLS roles (`tls_admin` or `tls_viewer`) via the `roles` field instead. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


