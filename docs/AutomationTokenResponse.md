# AutomationTokenResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the token. | 
**role** | Option<**String**> |  | 
**services** | Option<**Vec<String>**> | (Optional) The service IDs of the services the token will have access to. Separate service IDs with a space. If no services are specified, the token will have access to all services on the account.  | 
**scope** | Option<**String**> | A space-delimited list of authorization scope. | [default to Scope_Global]
**expires_at** | Option<**String**> | (optional) A UTC timestamp of when the token will expire. | 
**id** | Option<[**crate::models::ReadOnlyId**](ReadOnlyId.md)> |  | 
**customer_id** | Option<[**crate::models::ReadOnlyCustomerId**](ReadOnlyCustomerId.md)> |  | 
**ip** | Option<**String**> | The IP address of the client that last used the token. | 
**user_agent** | Option<**String**> | The User-Agent header of the client that last used the token. | 
**tls_access** | Option<**bool**> | Indicates whether TLS access is enabled for the token. | 
**last_used_at** | Option<**String**> | A UTC timestamp of when the token was last used. | [readonly]
**created_at** | Option<**String**> | A UTC timestamp of when the token was created. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


