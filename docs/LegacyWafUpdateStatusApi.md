# LegacyWafUpdateStatusApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_waf_update_status**](LegacyWafUpdateStatusApi.md#get_waf_update_status) | **GET** /service/{service_id}/wafs/{firewall_id}/update_statuses/{update_status_id} | Get the status of a WAF update
[**list_waf_update_statuses**](LegacyWafUpdateStatusApi.md#list_waf_update_statuses) | **GET** /service/{service_id}/wafs/{firewall_id}/update_statuses | List update statuses



## get_waf_update_status

Get a specific update status object for a particular service and firewall object.

```rust
let cfg = &Configuration::default();
let params = GetWafUpdateStatusParams {
    // parameters
};
get_waf_update_status(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |
**update_status_id** | **String** | Alphanumeric string identifying a WAF update status. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_waf_update_statuses

List all update statuses for a particular service and firewall object.

```rust
let cfg = &Configuration::default();
let params = ListWafUpdateStatusesParams {
    // parameters
};
list_waf_update_statuses(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**include** | Option\<**String**> | Include relationships. Optional, comma separated values. Permitted values: `waf`.  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

