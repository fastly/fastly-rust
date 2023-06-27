# AutomationTokenCreateResponseAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**crate::models::ReadOnlyId**](ReadOnlyId.md)> |  | 
**user_id** | Option<[**crate::models::ReadOnlyUserId**](ReadOnlyUserId.md)> |  | 
**customer_id** | Option<[**crate::models::ReadOnlyCustomerId**](ReadOnlyCustomerId.md)> |  | 
**sudo_expires_at** | Option<**String**> |  | [readonly]
**created_at** | Option<**String**> | A UTC time-stamp of when the token was created.  | [readonly]
**access_token** | Option<**String**> |  | [readonly]
**last_used_at** | Option<**String**> | A UTC time-stamp of when the token was last used. | [readonly]
**user_agent** | Option<**String**> | The User-Agent header of the client that last used the token. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


