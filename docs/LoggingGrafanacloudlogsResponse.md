# LoggingGrafanacloudlogsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name for the real-time logging configuration. | 
**placement** | Option<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  | 
**response_condition** | Option<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. | 
**format** | Option<**String**> | A Fastly [log format string](https://www.fastly.com/documentation/guides/integrations/streaming-logs/custom-log-formats/). | 
**log_processing_region** | Option<**String**> | The geographic region where the logs will be processed before streaming. Valid values are `us`, `eu`, and `none` for global. | [default to LogProcessingRegion_None]
**format_version** | Option<**String**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  | [default to FormatVersion_v2]
**user** | Option<**String**> | The Grafana Cloud Logs Dataset you want to log to. | 
**url** | Option<**String**> | The URL of the Loki instance in your Grafana stack. | 
**token** | Option<**String**> | The Grafana Access Policy token with `logs:write` access scoped to your Loki instance. | 
**index** | Option<**String**> | The Stream Labels, a JSON string used to identify the stream. | 
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**String**> |  | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


