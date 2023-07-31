# LoggingGenericCommonResponseAllOf1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**period** | Option<**String**> | How frequently log files are finalized so they can be available for reading (in seconds). | [default to 3600]
**gzip_level** | Option<**String**> | The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. | [default to 0]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


