# Server

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**weight** | Option<**i32**> | Weight (`1-100`) used to load balance this server against others. | [default to 100]
**max_conn** | Option<**i32**> | Maximum number of connections. If the value is `0`, it inherits the value from pool's `max_conn_default`. | [default to 0]
**port** | Option<**i32**> | Port number. Setting port `443` does not force TLS. Set `use_tls` in pool to force TLS. | [default to 80]
**address** | Option<**String**> | A hostname, IPv4, or IPv6 address for the server. Required. | 
**comment** | Option<**String**> | A freeform descriptive note. | 
**disabled** | Option<**bool**> | Allows servers to be enabled and disabled in a pool. | [default to false]
**override_host** | Option<**String**> | The hostname to override the Host header. Defaults to `null` meaning no override of the Host header if not set. This setting can also be added to a Pool definition. However, the server setting will override the Pool setting. | [default to null]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


