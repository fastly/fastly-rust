# ProductFanoutApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_fanout**](ProductFanoutApi.md#disable_product_fanout) | **DELETE** /enabled-products/v1/fanout/services/{service_id} | Disable product
[**enable_product_fanout**](ProductFanoutApi.md#enable_product_fanout) | **PUT** /enabled-products/v1/fanout/services/{service_id} | Enable product
[**get_product_fanout**](ProductFanoutApi.md#get_product_fanout) | **GET** /enabled-products/v1/fanout/services/{service_id} | Get product enablement status
[**get_services_product_fanout**](ProductFanoutApi.md#get_services_product_fanout) | **GET** /enabled-products/v1/fanout/services | Get services with product enabled



## disable_product_fanout

Disable the Fanout product on a service.

```rust
let cfg = &Configuration::default();
let params = DisableProductFanoutParams {
    // parameters
};
disable_product_fanout(cfg, params)
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


## enable_product_fanout

Enable the Fanout product on a service.

```rust
let cfg = &Configuration::default();
let params = EnableProductFanoutParams {
    // parameters
};
enable_product_fanout(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::FanoutResponseBodyEnable**](FanoutResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_fanout

Get the enablement status of the Fanout product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductFanoutParams {
    // parameters
};
get_product_fanout(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::FanoutResponseBodyEnable**](FanoutResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_services_product_fanout

Get all the services which have the Fanout product enabled.

```rust
let cfg = &Configuration::default();
let params = GetServicesProductFanoutParams {
    // parameters
};
get_services_product_fanout(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FanoutResponseBodyGetAllServices**](FanoutResponseBodyGetAllServices.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

