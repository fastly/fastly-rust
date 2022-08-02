# LoggingHttps

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name for the real-time logging configuration. | 
**placement** | Option<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  | 
**format_version** | Option<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  | [default to FormatVersion_v2]
**response_condition** | Option<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. | 
**format** | Option<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). | [default to %h %l %u %t "%r" %&gt;s %b]
**tls_ca_cert** | Option<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. | [default to null]
**tls_client_cert** | Option<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. | [default to null]
**tls_client_key** | Option<**String**> | The client private key used to make authenticated requests. Must be in PEM format. | [default to null]
**tls_hostname** | Option<**String**> | The hostname to verify the server's certificate. This should be one of the Subject Alternative Name (SAN) fields for the certificate. Common Names (CN) are not supported. | [default to null]
**request_max_entries** | Option<**i32**> | The maximum number of logs sent in one request. Defaults `0` (10k). | [default to 0]
**request_max_bytes** | Option<**i32**> | The maximum number of bytes sent in one request. Defaults `0` (100MB). | [default to 0]
**url** | Option<**String**> | The URL to send logs to. Must use HTTPS. Required. | 
**content_type** | Option<**String**> | Content type of the header sent with the request. | [default to null]
**header_name** | Option<**String**> | Name of the custom header sent with the request. | [default to null]
**message_type** | Option<[**crate::models::LoggingMessageType**](LoggingMessageType.md)> |  | 
**header_value** | Option<**String**> | Value of the custom header sent with the request. | [default to null]
**method** | Option<**String**> | HTTP method used for request. | [default to Method_POST]
**json_format** | Option<**String**> | Enforces valid JSON formatting for log entries. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


