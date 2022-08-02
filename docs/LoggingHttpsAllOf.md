# LoggingHttpsAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> | The URL to send logs to. Must use HTTPS. Required. | 
**request_max_entries** | Option<**i32**> | The maximum number of logs sent in one request. Defaults `0` (10k). | [default to 0]
**request_max_bytes** | Option<**i32**> | The maximum number of bytes sent in one request. Defaults `0` (100MB). | [default to 0]
**content_type** | Option<**String**> | Content type of the header sent with the request. | [default to null]
**header_name** | Option<**String**> | Name of the custom header sent with the request. | [default to null]
**message_type** | Option<[**crate::models::LoggingMessageType**](LoggingMessageType.md)> |  | 
**header_value** | Option<**String**> | Value of the custom header sent with the request. | [default to null]
**method** | Option<**String**> | HTTP method used for request. | [default to Method_POST]
**json_format** | Option<**String**> | Enforces valid JSON formatting for log entries. | 
**format** | Option<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). | [default to %h %l %u %t "%r" %&gt;s %b]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


