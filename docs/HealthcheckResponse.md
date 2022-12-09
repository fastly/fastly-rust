# HealthcheckResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**check_interval** | Option<**i32**> | How often to run the health check in milliseconds. | 
**comment** | Option<**String**> | A freeform descriptive note. | 
**expected_response** | Option<**i32**> | The status code expected from the host. | 
**headers** | Option<**Vec<String>**> | Array of custom headers that will be added to the health check probes. | 
**host** | Option<**String**> | Which host to check. | 
**http_version** | Option<**String**> | Whether to use version 1.0 or 1.1 HTTP. | 
**initial** | Option<**i32**> | When loading a config, the initial number of probes to be seen as OK. | 
**method** | Option<**String**> | Which HTTP method to use. | 
**name** | Option<**String**> | The name of the health check. | 
**path** | Option<**String**> | The path to check. | 
**threshold** | Option<**i32**> | How many health checks must succeed to be considered healthy. | 
**timeout** | Option<**i32**> | Timeout in milliseconds. | 
**window** | Option<**i32**> | The number of most recent health check queries to keep for this health check. | 
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**i32**> |  | [readonly]
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


