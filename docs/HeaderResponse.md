# HeaderResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> | Accepts a string value. | 
**cache_condition** | Option<**String**> | Name of the cache condition controlling when this configuration applies. | 
**dst** | Option<**String**> | Header to set. | 
**ignore_if_set** | Option<**i32**> | Don't add the header if it is added already. Only applies to 'set' action. | 
**name** | Option<**String**> | A handle to refer to this Header object. | 
**priority** | Option<**i32**> | Priority determines execution order. Lower numbers execute first. | [default to 100]
**regex** | Option<**String**> | Regular expression to use. Only applies to `regex` and `regex_repeat` actions. | 
**request_condition** | Option<**String**> | Condition which, if met, will select this configuration during a request. Optional. | 
**response_condition** | Option<**String**> | Optional name of a response condition to apply. | 
**src** | Option<**String**> | Variable to be used as a source for the header content. Does not apply to `delete` action. | 
**substitution** | Option<**String**> | Value to substitute in place of regular expression. Only applies to `regex` and `regex_repeat` actions. | 
**_type** | Option<**String**> | Accepts a string value. | 
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**i32**> |  | [readonly]
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


