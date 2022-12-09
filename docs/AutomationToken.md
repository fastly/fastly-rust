# AutomationToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the token. | 
**role** | Option<**String**> | The role on the token. | 
**services** | Option<**Vec<String>**> | (Optional) The service IDs of the services the token will have access to. Separate service IDs with a space. If no services are specified, the token will have access to all services on the account.  | 
**scope** | Option<**String**> | A space-delimited list of authorization scope. | [default to Scope_Global]
**expires_at** | Option<**String**> | A UTC time-stamp of when the token expires. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


