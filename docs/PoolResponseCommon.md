# PoolResponseCommon

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**between_bytes_timeout** | Option<**String**> | Maximum duration in milliseconds that Fastly will wait while receiving no data on a download from a backend. If exceeded, the response received so far will be considered complete and the fetch will end. May be set at runtime using `bereq.between_bytes_timeout`. | 
**connect_timeout** | Option<**String**> | How long to wait for a timeout in milliseconds. | 
**first_byte_timeout** | Option<**String**> | How long to wait for the first byte in milliseconds. | 
**max_conn_default** | Option<**String**> | Maximum number of connections. | [default to 200]
**tls_check_cert** | Option<**String**> | Be strict on checking TLS certs. | 
**id** | Option<**String**> |  | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


