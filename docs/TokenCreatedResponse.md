# TokenCreatedResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**services** | Option<**Vec<String>**> | List of alphanumeric strings identifying services (optional). If no services are specified, the token will have access to all services on the account.  | [readonly]
**name** | Option<**String**> | Name of the token. | 
**scope** | Option<**String**> | Space-delimited list of authorization scope. | [default to Scope_Global]
**created_at** | Option<**String**> | Time-stamp (UTC) of when the token was created. | 
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**id** | Option<**String**> |  | [readonly]
**user_id** | Option<**String**> |  | [readonly]
**last_used_at** | Option<**String**> | Time-stamp (UTC) of when the token was last used. | [readonly]
**expires_at** | Option<**String**> | Time-stamp (UTC) of when the token will expire (optional). | 
**ip** | Option<**String**> | IP Address of the client that last used the token. | 
**user_agent** | Option<**String**> | User-Agent header of the client that last used the token. | 
**access_token** | Option<**String**> | The alphanumeric string for accessing the API (only available on token creation). | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


