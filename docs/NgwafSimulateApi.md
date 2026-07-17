# NgwafSimulateApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**ngwaf_simulate_waf_request**](NgwafSimulateApi.md#ngwaf_simulate_waf_request) | **POST** /ngwaf/v1/workspaces/{workspace_id}/simulate | Simulate a WAF request



## ngwaf_simulate_waf_request

Simulates a request through the workspace's WAF configuration and returns the WAF response code and any signals that would be detected. The operation is stateless — no simulation data is persisted. 

```rust
let cfg = &Configuration::default();
let params = NgwafSimulateWafRequestParams {
    // parameters
};
ngwaf_simulate_waf_request(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The ID of the workspace. | [required] |
**waf_simulate_request** | [**WafSimulateRequest**](WafSimulateRequest.md) |  | [required] |

### Return type

[**crate::models::WafSimulateResponse**](WafSimulateResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

