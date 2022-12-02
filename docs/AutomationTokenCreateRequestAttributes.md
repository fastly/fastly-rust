# AutomationTokenCreateRequestAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | name of the token | 
**role** | Option<**String**> |  | 
**services** | Option<**Vec<String>**> | List of service ids to limit the token | 
**scope** | Option<**String**> |  | [default to Scope_Global]
**expires_at** | Option<**String**> | A UTC time-stamp of when the token will expire. | 
**tls_access** | Option<**bool**> | Indicates whether TLS access is enabled for the token. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


