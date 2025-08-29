# NgwafReportsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**get_attacks_report**](NgwafReportsApi.md#get_attacks_report) | **GET** /ngwaf/v1/reports/attacks | Get attacks report
[**get_signals_report**](NgwafReportsApi.md#get_signals_report) | **GET** /ngwaf/v1/reports/signals | Get signals report



## get_attacks_report

Get attacks report

```rust
let cfg = &Configuration::default();
let params = GetAttacksReportParams {
    // parameters
};
get_attacks_report(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | The start of a time range in RFC 3339 format. | [required] |
**to** | Option\<**String**> | The end of a time range in RFC 3339 format. Defaults to the current time. |  |

### Return type

[**crate::models::ListAttackReport**](ListAttackReport.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_signals_report

Get signals report

```rust
let cfg = &Configuration::default();
let params = GetSignalsReportParams {
    // parameters
};
get_signals_report(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | The start of a time range in RFC 3339 format. | [required] |
**to** | Option\<**String**> | The end of a time range in RFC 3339 format. Defaults to the current time. |  |
**signal_type** | Option\<**String**> | The type of signal |  |

### Return type

[**crate::models::ListSignalReport**](ListSignalReport.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

