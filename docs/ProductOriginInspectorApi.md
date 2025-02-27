# ProductOriginInspectorApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_origin_inspector**](ProductOriginInspectorApi.md#disable_product_origin_inspector) | **DELETE** /enabled-products/v1/origin_inspector/services/{service_id} | Disable product
[**enable_product_origin_inspector**](ProductOriginInspectorApi.md#enable_product_origin_inspector) | **PUT** /enabled-products/v1/origin_inspector/services/{service_id} | Enable product
[**get_product_origin_inspector**](ProductOriginInspectorApi.md#get_product_origin_inspector) | **GET** /enabled-products/v1/origin_inspector/services/{service_id} | Get product enablement status



## disable_product_origin_inspector

Disable the Origin Inspector product on a service.

```rust
let cfg = &Configuration::default();
let params = DisableProductOriginInspectorParams {
    // parameters
};
disable_product_origin_inspector(cfg, params)
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


## enable_product_origin_inspector

Enable the Origin Inspector product on a service.

```rust
let cfg = &Configuration::default();
let params = EnableProductOriginInspectorParams {
    // parameters
};
enable_product_origin_inspector(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::OriginInspectorResponseBodyEnable**](OriginInspectorResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_origin_inspector

Get the enablement status of the Origin Inspector product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductOriginInspectorParams {
    // parameters
};
get_product_origin_inspector(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::OriginInspectorResponseBodyEnable**](OriginInspectorResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

