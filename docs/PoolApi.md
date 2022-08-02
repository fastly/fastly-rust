# PoolApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_server_pool**](PoolApi.md#create_server_pool) | **POST** /service/{service_id}/version/{version_id}/pool | Create a server pool
[**delete_server_pool**](PoolApi.md#delete_server_pool) | **DELETE** /service/{service_id}/version/{version_id}/pool/{pool_name} | Delete a server pool
[**get_server_pool**](PoolApi.md#get_server_pool) | **GET** /service/{service_id}/version/{version_id}/pool/{pool_name} | Get a server pool
[**list_server_pools**](PoolApi.md#list_server_pools) | **GET** /service/{service_id}/version/{version_id}/pool | List server pools
[**update_server_pool**](PoolApi.md#update_server_pool) | **PUT** /service/{service_id}/version/{version_id}/pool/{pool_name} | Update a server pool



## create_server_pool

Creates a pool for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateServerPoolParams {
    // parameters
};
create_server_pool(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**tls_ca_cert** | Option\<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. |  |[default to null]
**tls_client_cert** | Option\<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_client_key** | Option\<**String**> | The client private key used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_cert_hostname** | Option\<**String**> | The hostname used to verify a server's certificate. It can either be the Common Name (CN) or a Subject Alternative Name (SAN). |  |[default to null]
**use_tls** | Option\<**i32**> | Whether to use TLS. |  |[default to 0]
**name** | Option\<**String**> | Name for the Pool. |  |
**shield** | Option\<**String**> | Selected POP to serve as a shield for the servers. Defaults to `null` meaning no origin shielding if not set. Refer to the [POPs API endpoint](/reference/api/utils/pops/) to get a list of available POPs used for shielding. |  |[default to null]
**request_condition** | Option\<**String**> | Condition which, if met, will select this configuration during a request. Optional. |  |
**max_conn_default** | Option\<**i32**> | Maximum number of connections. Optional. |  |[default to 200]
**connect_timeout** | Option\<**i32**> | How long to wait for a timeout in milliseconds. Optional. |  |
**first_byte_timeout** | Option\<**i32**> | How long to wait for the first byte in milliseconds. Optional. |  |
**quorum** | Option\<**i32**> | Percentage of capacity (`0-100`) that needs to be operationally available for a pool to be considered up. |  |[default to 75]
**tls_ciphers** | Option\<**String**> | List of OpenSSL ciphers (see the [openssl.org manpages](https://www.openssl.org/docs/man1.1.1/man1/ciphers.html) for details). Optional. |  |
**tls_sni_hostname** | Option\<**String**> | SNI hostname. Optional. |  |
**tls_check_cert** | Option\<**i32**> | Be strict on checking TLS certs. Optional. |  |
**min_tls_version** | Option\<**i32**> | Minimum allowed TLS version on connections to this server. Optional. |  |
**max_tls_version** | Option\<**i32**> | Maximum allowed TLS version on connections to this server. Optional. |  |
**healthcheck** | Option\<**String**> | Name of the healthcheck to use with this pool. Can be empty and could be reused across multiple backend and pools. |  |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**_type** | Option\<**String**> | What type of load balance group to use. |  |
**override_host** | Option\<**String**> | The hostname to [override the Host header](https://docs.fastly.com/en/guides/specifying-an-override-host). Defaults to `null` meaning no override of the Host header will occur. This setting can also be added to a Server definition. If the field is set on a Server definition it will override the Pool setting. |  |[default to null]

### Return type

[**crate::models::PoolResponse**](PoolResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_server_pool

Deletes a specific pool for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteServerPoolParams {
    // parameters
};
delete_server_pool(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**pool_name** | **String** | Name for the Pool. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_server_pool

Gets a single pool for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetServerPoolParams {
    // parameters
};
get_server_pool(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**pool_name** | **String** | Name for the Pool. | [required] |

### Return type

[**crate::models::PoolResponse**](PoolResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_server_pools

Lists all pools for a particular service and pool.

```rust
let cfg = &Configuration::default();
let params = ListServerPoolsParams {
    // parameters
};
list_server_pools(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::PoolResponse&gt;**](PoolResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_server_pool

Updates a specific pool for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateServerPoolParams {
    // parameters
};
update_server_pool(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**pool_name** | **String** | Name for the Pool. | [required] |
**tls_ca_cert** | Option\<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. |  |[default to null]
**tls_client_cert** | Option\<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_client_key** | Option\<**String**> | The client private key used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_cert_hostname** | Option\<**String**> | The hostname used to verify a server's certificate. It can either be the Common Name (CN) or a Subject Alternative Name (SAN). |  |[default to null]
**use_tls** | Option\<**i32**> | Whether to use TLS. |  |[default to UseTls_no_tls]
**name** | Option\<**String**> | Name for the Pool. |  |
**shield** | Option\<**String**> | Selected POP to serve as a shield for the servers. Defaults to `null` meaning no origin shielding if not set. Refer to the [POPs API endpoint](/reference/api/utils/pops/) to get a list of available POPs used for shielding. |  |[default to null]
**request_condition** | Option\<**String**> | Condition which, if met, will select this configuration during a request. Optional. |  |
**max_conn_default** | Option\<**i32**> | Maximum number of connections. Optional. |  |[default to 200]
**connect_timeout** | Option\<**i32**> | How long to wait for a timeout in milliseconds. Optional. |  |
**first_byte_timeout** | Option\<**i32**> | How long to wait for the first byte in milliseconds. Optional. |  |
**quorum** | Option\<**i32**> | Percentage of capacity (`0-100`) that needs to be operationally available for a pool to be considered up. |  |[default to 75]
**tls_ciphers** | Option\<**String**> | List of OpenSSL ciphers (see the [openssl.org manpages](https://www.openssl.org/docs/man1.1.1/man1/ciphers.html) for details). Optional. |  |
**tls_sni_hostname** | Option\<**String**> | SNI hostname. Optional. |  |
**tls_check_cert** | Option\<**i32**> | Be strict on checking TLS certs. Optional. |  |
**min_tls_version** | Option\<**i32**> | Minimum allowed TLS version on connections to this server. Optional. |  |
**max_tls_version** | Option\<**i32**> | Maximum allowed TLS version on connections to this server. Optional. |  |
**healthcheck** | Option\<**String**> | Name of the healthcheck to use with this pool. Can be empty and could be reused across multiple backend and pools. |  |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**_type** | Option\<**String**> | What type of load balance group to use. |  |
**override_host** | Option\<**String**> | The hostname to [override the Host header](https://docs.fastly.com/en/guides/specifying-an-override-host). Defaults to `null` meaning no override of the Host header will occur. This setting can also be added to a Server definition. If the field is set on a Server definition it will override the Pool setting. |  |[default to null]

### Return type

[**crate::models::PoolResponse**](PoolResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

