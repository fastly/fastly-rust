# AutomationTokenCreateResponseAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**crate::models::ReadOnlyId**](ReadOnlyId.md)> |  | 
**user_id** | Option<[**crate::models::ReadOnlyUserId**](ReadOnlyUserId.md)> |  | 
**customer_id** | Option<[**crate::models::ReadOnlyCustomerId**](ReadOnlyCustomerId.md)> |  | 
**created_at** | Option<**String**> | A UTC timestamp of when the token was created.  | [readonly]
**access_token** | Option<**String**> |  | [readonly]
**tls_access** | Option<**bool**> | Indicates whether TLS access is enabled for the token. | 
**last_used_at** | Option<**String**> | A UTC timestamp of when the token was last used. | [readonly]
**user_agent** | Option<**String**> | The User-Agent header of the client that last used the token. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


