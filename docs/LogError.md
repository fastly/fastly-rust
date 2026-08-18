# LogError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sequence_number** | Option<**i32**> | Sequence number for ordering messages. | 
**error_time_us** | Option<**i64**> | Timestamp of the error in microseconds. | 
**stream** | Option<**String**> | The stream type, always 'logging_error' for logging endpoint errors. | 
**message** | Option<**String**> | User-friendly error message. | 
**endpoint** | Option<**String**> | Name of the logging endpoint that generated the error. | 
**details** | Option<**String**> | Additional error details as a JSON string. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


