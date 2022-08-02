# LoggingKafka

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
**topic** | Option<**String**> | The Kafka topic to send logs to. Required. | 
**brokers** | Option<**String**> | A comma-separated list of IP addresses or hostnames of Kafka brokers. Required. | 
**compression_codec** | Option<**String**> | The codec used for compression of your logs. | 
**required_acks** | Option<**i32**> | The number of acknowledgements a leader must receive before a write is considered successful. | [default to RequiredAcks_one]
**request_max_bytes** | Option<**i32**> | The maximum number of bytes sent in one request. Defaults `0` (no limit). | [default to 0]
**parse_log_keyvals** | Option<**bool**> | Enables parsing of key=value tuples from the beginning of a logline, turning them into [record headers](https://cwiki.apache.org/confluence/display/KAFKA/KIP-82+-+Add+Record+Headers). | 
**auth_method** | Option<**String**> | SASL authentication method. | 
**user** | Option<**String**> | SASL user. | 
**password** | Option<**String**> | SASL password. | 
**use_tls** | Option<[**crate::models::LoggingUseTls**](LoggingUseTls.md)> |  | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


