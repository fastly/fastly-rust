# LegacyWafRulesetApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_waf_ruleset**](LegacyWafRulesetApi.md#get_waf_ruleset) | **GET** /service/{service_id}/wafs/{firewall_id}/ruleset | Get a WAF ruleset
[**get_waf_ruleset_vcl**](LegacyWafRulesetApi.md#get_waf_ruleset_vcl) | **GET** /service/{service_id}/wafs/{firewall_id}/ruleset/preview | Generate WAF ruleset VCL
[**update_waf_ruleset**](LegacyWafRulesetApi.md#update_waf_ruleset) | **PATCH** /service/{service_id}/wafs/{firewall_id}/ruleset | Update a WAF ruleset



## get_waf_ruleset

Get a WAF ruleset for a particular service and firewall object.

```rust
let cfg = &Configuration::default();
let params = GetWafRulesetParams {
    // parameters
};
get_waf_ruleset(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_waf_ruleset_vcl

Get a preview of the WAF ruleset VCL for a particular service and firewall object based on changes to WAF configuration before deploying the ruleset. The response will include a link to status of the background VCL generation job. Once the background job is completed, the preview WAF ruleset VCL can be retrieved from the status response.

```rust
let cfg = &Configuration::default();
let params = GetWafRulesetVclParams {
    // parameters
};
get_waf_ruleset_vcl(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_waf_ruleset

Update the WAF ruleset for a particular service and firewall object. Use the URL in the response to view the WAF ruleset deploy status.

```rust
let cfg = &Configuration::default();
let params = UpdateWafRulesetParams {
    // parameters
};
update_waf_ruleset(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

