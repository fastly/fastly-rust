# LoggingGcsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name for the real-time logging configuration. | 
**placement** | Option<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  | 
**format_version** | Option<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  | [default to FormatVersion_v2]
**response_condition** | Option<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. | 
**format** | Option<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). | [default to %h %l %u %t "%r" %&gt;s %b]
**message_type** | Option<**String**> | How the message should be formatted. | [default to MessageType_Classic]
**timestamp_format** | Option<**String**> | A timestamp format | [readonly]
**period** | Option<**i32**> | How frequently log files are finalized so they can be available for reading (in seconds). | [default to 3600]
**gzip_level** | Option<**i32**> | The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. | [default to 0]
**compression_codec** | Option<**String**> | The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. | 
**user** | Option<**String**> | Your Google Cloud Platform service account email address. The `client_email` field in your service account authentication JSON. Not required if `account_name` is specified. | 
**secret_key** | Option<**String**> | Your Google Cloud Platform account secret key. The `private_key` field in your service account authentication JSON. Not required if `account_name` is specified. | 
**account_name** | Option<**String**> | The name of the Google Cloud Platform service account associated with the target log collection service. Not required if `user` and `secret_key` are provided. | 
**bucket_name** | Option<**String**> | The name of the GCS bucket. | 
**path** | Option<**String**> | The path to upload logs to. | 
**public_key** | Option<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. | [default to null]
**project_id** | Option<**String**> | Your Google Cloud Platform project ID. Required | 
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**i32**> |  | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


