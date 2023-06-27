# AutomationTokenResponseAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**crate::models::ReadOnlyId**](ReadOnlyId.md)> |  | 
**customer_id** | Option<[**crate::models::ReadOnlyCustomerId**](ReadOnlyCustomerId.md)> |  | 
**role** | Option<**String**> |  | 
**ip** | Option<**String**> | The IP address of the client that last used the token. | 
**user_agent** | Option<**String**> | The User-Agent header of the client that last used the token. | 
**sudo_expires_at** | Option<**String**> |  | [readonly]
**last_used_at** | Option<**String**> | A UTC time-stamp of when the token was last used. | [readonly]
**created_at** | Option<**String**> | A UTC time-stamp of when the token was created. | 
**expires_at** | Option<**String**> | (optional) A UTC time-stamp of when the token will expire. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


