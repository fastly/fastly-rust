# BackendApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_backend**](BackendApi.md#create_backend) | **POST** /service/{service_id}/version/{version_id}/backend | Create a backend
[**delete_backend**](BackendApi.md#delete_backend) | **DELETE** /service/{service_id}/version/{version_id}/backend/{backend_name} | Delete a backend
[**get_backend**](BackendApi.md#get_backend) | **GET** /service/{service_id}/version/{version_id}/backend/{backend_name} | Describe a backend
[**list_backends**](BackendApi.md#list_backends) | **GET** /service/{service_id}/version/{version_id}/backend | List backends
[**update_backend**](BackendApi.md#update_backend) | **PUT** /service/{service_id}/version/{version_id}/backend/{backend_name} | Update a backend



## create_backend

Create a backend for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateBackendParams {
    // parameters
};
create_backend(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**address** | Option\<**String**> | A hostname, IPv4, or IPv6 address for the backend. This is the preferred way to specify the location of your backend. |  |
**auto_loadbalance** | Option\<**bool**> | Whether or not this backend should be automatically load balanced. If true, all backends with this setting that don't have a `request_condition` will be selected based on their `weight`. |  |
**between_bytes_timeout** | Option\<**i32**> | Maximum duration in milliseconds that Fastly will wait while receiving no data on a download from a backend. If exceeded, the response received so far will be considered complete and the fetch will end. May be set at runtime using `bereq.between_bytes_timeout`. |  |
**client_cert** | Option\<**String**> | Unused. |  |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**connect_timeout** | Option\<**i32**> | Maximum duration in milliseconds to wait for a connection to this backend to be established. If exceeded, the connection is aborted and a synthethic `503` response will be presented instead. May be set at runtime using `bereq.connect_timeout`. |  |
**first_byte_timeout** | Option\<**i32**> | Maximum duration in milliseconds to wait for the server response to begin after a TCP connection is established and the request has been sent. If exceeded, the connection is aborted and a synthethic `503` response will be presented instead. May be set at runtime using `bereq.first_byte_timeout`. |  |
**healthcheck** | Option\<**String**> | The name of the healthcheck to use with this backend. |  |
**hostname** | Option\<**String**> | The hostname of the backend. May be used as an alternative to `address` to set the backend location. |  |
**ipv4** | Option\<**String**> | IPv4 address of the backend. May be used as an alternative to `address` to set the backend location. |  |
**ipv6** | Option\<**String**> | IPv6 address of the backend. May be used as an alternative to `address` to set the backend location. |  |
**keepalive_time** | Option\<**i32**> | How long in seconds to keep a persistent connection to the backend between requests. |  |
**max_conn** | Option\<**i32**> | Maximum number of concurrent connections this backend will accept. |  |
**max_tls_version** | Option\<**String**> | Maximum allowed TLS version on SSL connections to this backend. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated. |  |
**min_tls_version** | Option\<**String**> | Minimum allowed TLS version on SSL connections to this backend. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated. |  |
**name** | Option\<**String**> | The name of the backend. |  |
**override_host** | Option\<**String**> | If set, will replace the client-supplied HTTP `Host` header on connections to this backend. Applied after VCL has been processed, so this setting will take precedence over changing `bereq.http.Host` in VCL. |  |
**port** | Option\<**i32**> | Port on which the backend server is listening for connections from Fastly. Setting `port` to 80 or 443 will also set `use_ssl` automatically (to false and true respectively), unless explicitly overridden by setting `use_ssl` in the same request. |  |
**request_condition** | Option\<**String**> | Name of a Condition, which if satisfied, will select this backend during a request. If set, will override any `auto_loadbalance` setting. By default, the first backend added to a service is selected for all requests. |  |
**shield** | Option\<**String**> | Identifier of the POP to use as a [shield](https://docs.fastly.com/en/guides/shielding). |  |
**ssl_ca_cert** | Option\<**String**> | CA certificate attached to origin. |  |
**ssl_cert_hostname** | Option\<**String**> | Overrides `ssl_hostname`, but only for cert verification. Does not affect SNI at all. |  |
**ssl_check_cert** | Option\<**bool**> | Be strict on checking SSL certs. |  |[default to true]
**ssl_ciphers** | Option\<**String**> | List of [OpenSSL ciphers](https://www.openssl.org/docs/manmaster/man1/ciphers.html) to support for connections to this origin. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated. |  |
**ssl_client_cert** | Option\<**String**> | Client certificate attached to origin. |  |
**ssl_client_key** | Option\<**String**> | Client key attached to origin. |  |
**ssl_hostname** | Option\<**String**> | Use `ssl_cert_hostname` and `ssl_sni_hostname` to configure certificate validation. |  |
**ssl_sni_hostname** | Option\<**String**> | Overrides `ssl_hostname`, but only for SNI in the handshake. Does not affect cert validation at all. |  |
**use_ssl** | Option\<**bool**> | Whether or not to require TLS for connections to this backend. |  |
**weight** | Option\<**i32**> | Weight used to load balance this backend against others. May be any positive integer. If `auto_loadbalance` is true, the chance of this backend being selected is equal to its own weight over the sum of all weights for backends that have `auto_loadbalance` set to true. |  |

### Return type

[**crate::models::BackendResponse**](BackendResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_backend

Delete the backend for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteBackendParams {
    // parameters
};
delete_backend(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**backend_name** | **String** | The name of the backend. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_backend

Get the backend for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetBackendParams {
    // parameters
};
get_backend(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**backend_name** | **String** | The name of the backend. | [required] |

### Return type

[**crate::models::BackendResponse**](BackendResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_backends

List all backends for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListBackendsParams {
    // parameters
};
list_backends(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::BackendResponse&gt;**](BackendResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_backend

Update the backend for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateBackendParams {
    // parameters
};
update_backend(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**backend_name** | **String** | The name of the backend. | [required] |
**address** | Option\<**String**> | A hostname, IPv4, or IPv6 address for the backend. This is the preferred way to specify the location of your backend. |  |
**auto_loadbalance** | Option\<**bool**> | Whether or not this backend should be automatically load balanced. If true, all backends with this setting that don't have a `request_condition` will be selected based on their `weight`. |  |
**between_bytes_timeout** | Option\<**i32**> | Maximum duration in milliseconds that Fastly will wait while receiving no data on a download from a backend. If exceeded, the response received so far will be considered complete and the fetch will end. May be set at runtime using `bereq.between_bytes_timeout`. |  |
**client_cert** | Option\<**String**> | Unused. |  |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**connect_timeout** | Option\<**i32**> | Maximum duration in milliseconds to wait for a connection to this backend to be established. If exceeded, the connection is aborted and a synthethic `503` response will be presented instead. May be set at runtime using `bereq.connect_timeout`. |  |
**first_byte_timeout** | Option\<**i32**> | Maximum duration in milliseconds to wait for the server response to begin after a TCP connection is established and the request has been sent. If exceeded, the connection is aborted and a synthethic `503` response will be presented instead. May be set at runtime using `bereq.first_byte_timeout`. |  |
**healthcheck** | Option\<**String**> | The name of the healthcheck to use with this backend. |  |
**hostname** | Option\<**String**> | The hostname of the backend. May be used as an alternative to `address` to set the backend location. |  |
**ipv4** | Option\<**String**> | IPv4 address of the backend. May be used as an alternative to `address` to set the backend location. |  |
**ipv6** | Option\<**String**> | IPv6 address of the backend. May be used as an alternative to `address` to set the backend location. |  |
**keepalive_time** | Option\<**i32**> | How long in seconds to keep a persistent connection to the backend between requests. |  |
**max_conn** | Option\<**i32**> | Maximum number of concurrent connections this backend will accept. |  |
**max_tls_version** | Option\<**String**> | Maximum allowed TLS version on SSL connections to this backend. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated. |  |
**min_tls_version** | Option\<**String**> | Minimum allowed TLS version on SSL connections to this backend. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated. |  |
**name** | Option\<**String**> | The name of the backend. |  |
**override_host** | Option\<**String**> | If set, will replace the client-supplied HTTP `Host` header on connections to this backend. Applied after VCL has been processed, so this setting will take precedence over changing `bereq.http.Host` in VCL. |  |
**port** | Option\<**i32**> | Port on which the backend server is listening for connections from Fastly. Setting `port` to 80 or 443 will also set `use_ssl` automatically (to false and true respectively), unless explicitly overridden by setting `use_ssl` in the same request. |  |
**request_condition** | Option\<**String**> | Name of a Condition, which if satisfied, will select this backend during a request. If set, will override any `auto_loadbalance` setting. By default, the first backend added to a service is selected for all requests. |  |
**shield** | Option\<**String**> | Identifier of the POP to use as a [shield](https://docs.fastly.com/en/guides/shielding). |  |
**ssl_ca_cert** | Option\<**String**> | CA certificate attached to origin. |  |
**ssl_cert_hostname** | Option\<**String**> | Overrides `ssl_hostname`, but only for cert verification. Does not affect SNI at all. |  |
**ssl_check_cert** | Option\<**bool**> | Be strict on checking SSL certs. |  |[default to true]
**ssl_ciphers** | Option\<**String**> | List of [OpenSSL ciphers](https://www.openssl.org/docs/manmaster/man1/ciphers.html) to support for connections to this origin. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated. |  |
**ssl_client_cert** | Option\<**String**> | Client certificate attached to origin. |  |
**ssl_client_key** | Option\<**String**> | Client key attached to origin. |  |
**ssl_hostname** | Option\<**String**> | Use `ssl_cert_hostname` and `ssl_sni_hostname` to configure certificate validation. |  |
**ssl_sni_hostname** | Option\<**String**> | Overrides `ssl_hostname`, but only for SNI in the handshake. Does not affect cert validation at all. |  |
**use_ssl** | Option\<**bool**> | Whether or not to require TLS for connections to this backend. |  |
**weight** | Option\<**i32**> | Weight used to load balance this backend against others. May be any positive integer. If `auto_loadbalance` is true, the chance of this backend being selected is equal to its own weight over the sum of all weights for backends that have `auto_loadbalance` set to true. |  |

### Return type

[**crate::models::BackendResponse**](BackendResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

