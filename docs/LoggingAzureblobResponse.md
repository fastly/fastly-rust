# LoggingAzureblobResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name for the real-time logging configuration. | 
**placement** | Option<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  | 
**response_condition** | Option<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. | 
**format** | Option<**String**> | A Fastly [log format string](https://www.fastly.com/documentation/guides/integrations/streaming-logs/custom-log-formats/). | [default to %h %l %u %t "%r" %&gt;s %b]
**log_processing_region** | Option<**String**> | The geographic region where the logs will be processed before streaming. Valid values are `us`, `eu`, and `none` for global. | [default to LogProcessingRegion_None]
**format_version** | Option<**String**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  | [default to FormatVersion_v2]
**message_type** | Option<**String**> | How the message should be formatted. | [default to MessageType_Classic]
**timestamp_format** | Option<**String**> | A timestamp format | [readonly]
**compression_codec** | Option<**String**> | The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. | 
**period** | Option<**String**> | How frequently log files are finalized so they can be available for reading (in seconds). | [default to 3600]
**gzip_level** | Option<**String**> | The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. | [default to 0]
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**String**> |  | [readonly]
**path** | Option<**String**> | The path to upload logs to. | [default to null]
**account_name** | Option<**String**> | The unique Azure Blob Storage namespace in which your data objects are stored. Required. | 
**container** | Option<**String**> | The name of the Azure Blob Storage container in which to store logs. Required. | 
**sas_token** | Option<**String**> | The Azure shared access signature providing write access to the blob service objects. Be sure to update your token before it expires or the logging functionality will not work. Required. | 
**public_key** | Option<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. | [default to null]
**file_max_bytes** | Option<**i32**> | The maximum number of bytes for each uploaded file. A value of 0 can be used to indicate there is no limit on the size of uploaded files, otherwise the minimum value is 1048576 bytes (1 MiB). Note that Microsoft Azure Storage has [block size limits](https://learn.microsoft.com/en-us/rest/api/storageservices/put-block?tabs=microsoft-entra-id#remarks). | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


