# LoggingDigitalocean

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
**bucket_name** | Option<**String**> | The name of the DigitalOcean Space. | 
**access_key** | Option<**String**> | Your DigitalOcean Spaces account access key. | 
**secret_key** | Option<**String**> | Your DigitalOcean Spaces account secret key. | 
**domain** | Option<**String**> | The domain of the DigitalOcean Spaces endpoint. | [default to nyc3.digitaloceanspaces.com]
**path** | Option<**String**> | The path to upload logs to. | [default to null]
**public_key** | Option<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. | [default to null]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


