# AutomationTokenCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the token. | 
**role** | Option<**String**> | The role on the token. | 
**services** | Option<**Vec<String>**> | (Optional) The service IDs of the services the token will have access to. Separate service IDs with a space. If no services are specified, the token will have access to all services on the account.  | 
**scope** | Option<**String**> | A space-delimited list of authorization scope. | [default to Scope_Global]
**expires_at** | Option<**String**> | A UTC time-stamp of when the token expires. | 
**created_at** | Option<**String**> | A UTC time-stamp of when the token was created.  | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**id** | Option<[**crate::models::ReadOnlyId**](ReadOnlyId.md)> |  | 
**user_id** | Option<[**crate::models::ReadOnlyUserId**](ReadOnlyUserId.md)> |  | 
**customer_id** | Option<[**crate::models::ReadOnlyCustomerId**](ReadOnlyCustomerId.md)> |  | 
**sudo_expires_at** | Option<**String**> |  | [readonly]
**access_token** | Option<**String**> |  | [readonly]
**last_used_at** | Option<**String**> | A UTC time-stamp of when the token was last used. | [readonly]
**user_agent** | Option<**String**> | The User-Agent header of the client that last used the token. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


