# LoggingElasticsearchResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name for the real-time logging configuration. | 
**placement** | Option<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  | 
**response_condition** | Option<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. | 
**format** | Option<**String**> | A Fastly [log format string](https://www.fastly.com/documentation/guides/integrations/streaming-logs/custom-log-formats/). Must produce valid JSON that Elasticsearch can ingest. | 
**log_processing_region** | Option<**String**> | The geographic region where the logs will be processed before streaming. Valid values are `us`, `eu`, and `none` for global. | [default to LogProcessingRegion_None]
**format_version** | Option<**String**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  | [default to FormatVersion_v2]
**tls_ca_cert** | Option<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. | [default to null]
**tls_client_cert** | Option<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. | [default to null]
**tls_client_key** | Option<**String**> | The client private key used to make authenticated requests. Must be in PEM format. | [default to null]
**tls_hostname** | Option<**String**> | The hostname to verify the server's certificate. This should be one of the Subject Alternative Name (SAN) fields for the certificate. Common Names (CN) are not supported. | [default to null]
**request_max_entries** | Option<**i32**> | The maximum number of logs sent in one request. Defaults `0` for unbounded. | [default to 0]
**request_max_bytes** | Option<**i32**> | The maximum number of bytes sent in one request. Defaults `0` for unbounded. | [default to 0]
**index** | Option<**String**> | The name of the Elasticsearch index to send documents (logs) to. The index must follow the Elasticsearch [index format rules](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html). We support [strftime](https://www.man7.org/linux/man-pages/man3/strftime.3.html) interpolated variables inside braces prefixed with a pound symbol. For example, `#{%F}` will interpolate as `YYYY-MM-DD` with today's date. | 
**url** | Option<**String**> | The URL to stream logs to. Must use HTTPS. | 
**pipeline** | Option<**String**> | The ID of the Elasticsearch ingest pipeline to apply pre-process transformations to before indexing. Learn more about creating a pipeline in the [Elasticsearch docs](https://www.elastic.co/guide/en/elasticsearch/reference/current/ingest.html). | 
**user** | Option<**String**> | Basic Auth username. | 
**password** | Option<**String**> | Basic Auth password. | 
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**String**> |  | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


