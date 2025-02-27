# ProductDomainInspectorApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_domain_inspector**](ProductDomainInspectorApi.md#disable_product_domain_inspector) | **DELETE** /enabled-products/v1/domain_inspector/services/{service_id} | Disable product
[**enable_product_domain_inspector**](ProductDomainInspectorApi.md#enable_product_domain_inspector) | **PUT** /enabled-products/v1/domain_inspector/services/{service_id} | Enable product
[**get_product_domain_inspector**](ProductDomainInspectorApi.md#get_product_domain_inspector) | **GET** /enabled-products/v1/domain_inspector/services/{service_id} | Get product enablement status



## disable_product_domain_inspector

Disable the Domain Inspector product on a service.

```rust
let cfg = &Configuration::default();
let params = DisableProductDomainInspectorParams {
    // parameters
};
disable_product_domain_inspector(cfg, params)
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


## enable_product_domain_inspector

Enable the Domain Inspector product on a service.

```rust
let cfg = &Configuration::default();
let params = EnableProductDomainInspectorParams {
    // parameters
};
enable_product_domain_inspector(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::DomainInspectorResponseBodyEnable**](DomainInspectorResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_domain_inspector

Get the enablement status of the Domain Inspector product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductDomainInspectorParams {
    // parameters
};
get_product_domain_inspector(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::DomainInspectorResponseBodyEnable**](DomainInspectorResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

