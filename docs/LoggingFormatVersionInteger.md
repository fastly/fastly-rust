# LoggingFormatVersionInteger

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**format_version** | Option<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  | [default to FormatVersion_v2]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


