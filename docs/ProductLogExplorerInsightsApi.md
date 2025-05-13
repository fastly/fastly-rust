# ProductLogExplorerInsightsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_log_explorer_insights**](ProductLogExplorerInsightsApi.md#disable_product_log_explorer_insights) | **DELETE** /enabled-products/v1/log_explorer_insights/services/{service_id} | Disable product
[**enable_product_log_explorer_insights**](ProductLogExplorerInsightsApi.md#enable_product_log_explorer_insights) | **PUT** /enabled-products/v1/log_explorer_insights/services/{service_id} | Enable product
[**get_product_log_explorer_insights**](ProductLogExplorerInsightsApi.md#get_product_log_explorer_insights) | **GET** /enabled-products/v1/log_explorer_insights/services/{service_id} | Get product enablement status
[**get_services_product_log_explorer_insights**](ProductLogExplorerInsightsApi.md#get_services_product_log_explorer_insights) | **GET** /enabled-products/v1/log_explorer_insights/services | Get services with product enabled



## disable_product_log_explorer_insights

Disable the Log Explorer & Insights product on a service.

```rust
let cfg = &Configuration::default();
let params = DisableProductLogExplorerInsightsParams {
    // parameters
};
disable_product_log_explorer_insights(cfg, params)
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


## enable_product_log_explorer_insights

Enable the Log Explorer & Insights product on a service.

```rust
let cfg = &Configuration::default();
let params = EnableProductLogExplorerInsightsParams {
    // parameters
};
enable_product_log_explorer_insights(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::LogExplorerInsightsResponseBodyEnable**](LogExplorerInsightsResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_log_explorer_insights

Get the enablement status of the Log Explorer & Insights product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductLogExplorerInsightsParams {
    // parameters
};
get_product_log_explorer_insights(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::LogExplorerInsightsResponseBodyEnable**](LogExplorerInsightsResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_services_product_log_explorer_insights

Get all the services which have the Log Explorer & Insights product enabled.

```rust
let cfg = &Configuration::default();
let params = GetServicesProductLogExplorerInsightsParams {
    // parameters
};
get_services_product_log_explorer_insights(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LogExplorerInsightsResponseBodyGetAllServices**](LogExplorerInsightsResponseBodyGetAllServices.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

