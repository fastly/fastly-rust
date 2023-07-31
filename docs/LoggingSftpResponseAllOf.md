# LoggingSftpResponseAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | A hostname or IPv4 address. | 
**port** | Option<**String**> | The port number. | [default to 22]
**period** | Option<**String**> | How frequently log files are finalized so they can be available for reading (in seconds). | [default to 3600]
**gzip_level** | Option<**i32**> | The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. | [default to 0]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


