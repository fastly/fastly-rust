# AutomationTokenCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [readonly]
**user_id** | Option<**String**> |  | 
**customer_id** | Option<**String**> |  | 
**name** | Option<**String**> |  | 
**role** | Option<**String**> |  | 
**scopes** | Option<**String**> |  | [default to Scopes_Global]
**scope** | Option<**String**> |  | 
**services** | Option<**Vec<String>**> | List of alphanumeric strings identifying services (optional). If no services are specified, the token will have access to all services on the account. | 
**sudo_expires_at** | Option<**String**> |  | 
**created_at** | Option<**String**> | Time-stamp (UTC) of when the token was created.  | 
**expires_at** | Option<**String**> |  | 
**object** | Option<**String**> |  | 
**access_token** | Option<**String**> |  | 
**last_used_at** | Option<**String**> | Time-stamp (UTC) of when the token was last used. | 
**user_agent** | Option<**String**> | User-Agent header of the client that last used the token. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


