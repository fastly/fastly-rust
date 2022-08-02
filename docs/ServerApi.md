# ServerApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pool_server**](ServerApi.md#create_pool_server) | **POST** /service/{service_id}/pool/{pool_id}/server | Add a server to a pool
[**delete_pool_server**](ServerApi.md#delete_pool_server) | **DELETE** /service/{service_id}/pool/{pool_id}/server/{server_id} | Delete a server from a pool
[**get_pool_server**](ServerApi.md#get_pool_server) | **GET** /service/{service_id}/pool/{pool_id}/server/{server_id} | Get a pool server
[**list_pool_servers**](ServerApi.md#list_pool_servers) | **GET** /service/{service_id}/pool/{pool_id}/servers | List servers in a pool
[**update_pool_server**](ServerApi.md#update_pool_server) | **PUT** /service/{service_id}/pool/{pool_id}/server/{server_id} | Update a server



## create_pool_server

Creates a single server for a particular service and pool.

```rust
let cfg = &Configuration::default();
let params = CreatePoolServerParams {
    // parameters
};
create_pool_server(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**pool_id** | **String** | Alphanumeric string identifying a Pool. | [required] |
**weight** | Option\<**i32**> | Weight (`1-100`) used to load balance this server against others. |  |[default to 100]
**max_conn** | Option\<**i32**> | Maximum number of connections. If the value is `0`, it inherits the value from pool's `max_conn_default`. |  |[default to 0]
**port** | Option\<**i32**> | Port number. Setting port `443` does not force TLS. Set `use_tls` in pool to force TLS. |  |[default to 80]
**address** | Option\<**String**> | A hostname, IPv4, or IPv6 address for the server. Required. |  |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**disabled** | Option\<**bool**> | Allows servers to be enabled and disabled in a pool. |  |[default to false]
**override_host** | Option\<**String**> | The hostname to override the Host header. Defaults to `null` meaning no override of the Host header if not set. This setting can also be added to a Pool definition. However, the server setting will override the Pool setting. |  |[default to null]

### Return type

[**crate::models::ServerResponse**](ServerResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_pool_server

Deletes a single server for a particular service and pool.

```rust
let cfg = &Configuration::default();
let params = DeletePoolServerParams {
    // parameters
};
delete_pool_server(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**pool_id** | **String** | Alphanumeric string identifying a Pool. | [required] |
**server_id** | **String** | Alphanumeric string identifying a Server. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_pool_server

Gets a single server for a particular service and pool.

```rust
let cfg = &Configuration::default();
let params = GetPoolServerParams {
    // parameters
};
get_pool_server(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**pool_id** | **String** | Alphanumeric string identifying a Pool. | [required] |
**server_id** | **String** | Alphanumeric string identifying a Server. | [required] |

### Return type

[**crate::models::ServerResponse**](ServerResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_pool_servers

Lists all servers for a particular service and pool.

```rust
let cfg = &Configuration::default();
let params = ListPoolServersParams {
    // parameters
};
list_pool_servers(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**pool_id** | **String** | Alphanumeric string identifying a Pool. | [required] |

### Return type

[**Vec&lt;crate::models::ServerResponse&gt;**](ServerResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_pool_server

Updates a single server for a particular service and pool.

```rust
let cfg = &Configuration::default();
let params = UpdatePoolServerParams {
    // parameters
};
update_pool_server(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**pool_id** | **String** | Alphanumeric string identifying a Pool. | [required] |
**server_id** | **String** | Alphanumeric string identifying a Server. | [required] |
**weight** | Option\<**i32**> | Weight (`1-100`) used to load balance this server against others. |  |[default to 100]
**max_conn** | Option\<**i32**> | Maximum number of connections. If the value is `0`, it inherits the value from pool's `max_conn_default`. |  |[default to 0]
**port** | Option\<**i32**> | Port number. Setting port `443` does not force TLS. Set `use_tls` in pool to force TLS. |  |[default to 80]
**address** | Option\<**String**> | A hostname, IPv4, or IPv6 address for the server. Required. |  |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**disabled** | Option\<**bool**> | Allows servers to be enabled and disabled in a pool. |  |[default to false]
**override_host** | Option\<**String**> | The hostname to override the Host header. Defaults to `null` meaning no override of the Host header if not set. This setting can also be added to a Pool definition. However, the server setting will override the Pool setting. |  |[default to null]

### Return type

[**crate::models::ServerResponse**](ServerResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

