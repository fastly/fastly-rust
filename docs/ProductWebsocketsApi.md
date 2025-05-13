# ProductWebsocketsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_websockets**](ProductWebsocketsApi.md#disable_product_websockets) | **DELETE** /enabled-products/v1/websockets/services/{service_id} | Disable product
[**enable_product_websockets**](ProductWebsocketsApi.md#enable_product_websockets) | **PUT** /enabled-products/v1/websockets/services/{service_id} | Enable product
[**get_product_websockets**](ProductWebsocketsApi.md#get_product_websockets) | **GET** /enabled-products/v1/websockets/services/{service_id} | Get product enablement status
[**get_services_product_websockets**](ProductWebsocketsApi.md#get_services_product_websockets) | **GET** /enabled-products/v1/websockets/services | Get services with product enabled



## disable_product_websockets

Disable the Websockets product on a service.

```rust
let cfg = &Configuration::default();
let params = DisableProductWebsocketsParams {
    // parameters
};
disable_product_websockets(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## enable_product_websockets

Enable the WebSockets product on a service.

```rust
let cfg = &Configuration::default();
let params = EnableProductWebsocketsParams {
    // parameters
};
enable_product_websockets(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::WebsocketsResponseBodyEnable**](WebsocketsResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_websockets

Get the enablement status of the WebSockets product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductWebsocketsParams {
    // parameters
};
get_product_websockets(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::WebsocketsResponseBodyEnable**](WebsocketsResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_services_product_websockets

Get all the services which have the Websockets product enabled.

```rust
let cfg = &Configuration::default();
let params = GetServicesProductWebsocketsParams {
    // parameters
};
get_services_product_websockets(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WebsocketsResponseBodyGetAllServices**](WebsocketsResponseBodyGetAllServices.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

