# LoggingGenericCommonResponseAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message_type** | Option<**String**> | How the message should be formatted. | [default to MessageType_Classic]
**timestamp_format** | Option<**String**> | A timestamp format | [readonly]
**compression_codec** | Option<**String**> | The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


