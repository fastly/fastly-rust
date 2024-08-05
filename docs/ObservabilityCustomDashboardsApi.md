# ObservabilityCustomDashboardsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**create_dashboard**](ObservabilityCustomDashboardsApi.md#create_dashboard) | **POST** /observability/dashboards | Create a new dashboard
[**delete_dashboard**](ObservabilityCustomDashboardsApi.md#delete_dashboard) | **DELETE** /observability/dashboards/{dashboard_id} | Delete an existing dashboard
[**get_dashboard**](ObservabilityCustomDashboardsApi.md#get_dashboard) | **GET** /observability/dashboards/{dashboard_id} | Retrieve a dashboard by ID
[**list_dashboards**](ObservabilityCustomDashboardsApi.md#list_dashboards) | **GET** /observability/dashboards | List all custom dashboards
[**update_dashboard**](ObservabilityCustomDashboardsApi.md#update_dashboard) | **PATCH** /observability/dashboards/{dashboard_id} | Update an existing dashboard



## create_dashboard

Create a new dashboard

```rust
let cfg = &Configuration::default();
let params = CreateDashboardParams {
    // parameters
};
create_dashboard(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_dashboard_request** | Option\<[**CreateDashboardRequest**](CreateDashboardRequest.md)> |  |  |

### Return type

[**crate::models::Dashboard**](Dashboard.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_dashboard

Delete an existing dashboard

```rust
let cfg = &Configuration::default();
let params = DeleteDashboardParams {
    // parameters
};
delete_dashboard(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_dashboard

Retrieve a dashboard by ID

```rust
let cfg = &Configuration::default();
let params = GetDashboardParams {
    // parameters
};
get_dashboard(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** |  | [required] |

### Return type

[**crate::models::Dashboard**](Dashboard.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_dashboards

List all custom dashboards

```rust
let cfg = &Configuration::default();
let params = ListDashboardsParams {
    // parameters
};
list_dashboards(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListDashboardsResponse**](ListDashboardsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_dashboard

Update an existing dashboard

```rust
let cfg = &Configuration::default();
let params = UpdateDashboardParams {
    // parameters
};
update_dashboard(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** |  | [required] |
**update_dashboard_request** | Option\<[**UpdateDashboardRequest**](UpdateDashboardRequest.md)> |  |  |

### Return type

[**crate::models::Dashboard**](Dashboard.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

