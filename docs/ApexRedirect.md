# ApexRedirect

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**i32**> |  | [readonly]
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**status_code** | Option<**i32**> | HTTP status code used to redirect the client. | 
**domains** | Option<**Vec<String>**> | Array of apex domains that should redirect to their WWW subdomain. | 
**feature_revision** | Option<**i32**> | Revision number of the apex redirect feature implementation. Defaults to the most recent revision. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


