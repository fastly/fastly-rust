# WafExclusionsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_waf_rule_exclusion**](WafExclusionsApi.md#create_waf_rule_exclusion) | **POST** /waf/firewalls/{firewall_id}/versions/{firewall_version_number}/exclusions | Create a WAF rule exclusion
[**delete_waf_rule_exclusion**](WafExclusionsApi.md#delete_waf_rule_exclusion) | **DELETE** /waf/firewalls/{firewall_id}/versions/{firewall_version_number}/exclusions/{exclusion_number} | Delete a WAF rule exclusion
[**get_waf_rule_exclusion**](WafExclusionsApi.md#get_waf_rule_exclusion) | **GET** /waf/firewalls/{firewall_id}/versions/{firewall_version_number}/exclusions/{exclusion_number} | Get a WAF rule exclusion
[**list_waf_rule_exclusions**](WafExclusionsApi.md#list_waf_rule_exclusions) | **GET** /waf/firewalls/{firewall_id}/versions/{firewall_version_number}/exclusions | List WAF rule exclusions
[**update_waf_rule_exclusion**](WafExclusionsApi.md#update_waf_rule_exclusion) | **PATCH** /waf/firewalls/{firewall_id}/versions/{firewall_version_number}/exclusions/{exclusion_number} | Update a WAF rule exclusion



## create_waf_rule_exclusion

Create a WAF exclusion for a particular firewall version.

```rust
let cfg = &Configuration::default();
let params = CreateWafRuleExclusionParams {
    // parameters
};
create_waf_rule_exclusion(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**firewall_version_number** | **i32** | Integer identifying a WAF firewall version. | [required] |
**waf_exclusion** | Option\<[**WafExclusion**](WafExclusion.md)> |  |  |

### Return type

[**crate::models::WafExclusionResponse**](WafExclusionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_waf_rule_exclusion

Delete a WAF exclusion for a particular firewall version.

```rust
let cfg = &Configuration::default();
let params = DeleteWafRuleExclusionParams {
    // parameters
};
delete_waf_rule_exclusion(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**firewall_version_number** | **i32** | Integer identifying a WAF firewall version. | [required] |
**exclusion_number** | **i32** | A numeric ID identifying a WAF exclusion. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_waf_rule_exclusion

Get a specific WAF exclusion object.

```rust
let cfg = &Configuration::default();
let params = GetWafRuleExclusionParams {
    // parameters
};
get_waf_rule_exclusion(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**firewall_version_number** | **i32** | Integer identifying a WAF firewall version. | [required] |
**exclusion_number** | **i32** | A numeric ID identifying a WAF exclusion. | [required] |

### Return type

[**crate::models::WafExclusionResponse**](WafExclusionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_waf_rule_exclusions

List all exclusions for a particular firewall version.

```rust
let cfg = &Configuration::default();
let params = ListWafRuleExclusionsParams {
    // parameters
};
list_waf_rule_exclusions(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**firewall_version_number** | **i32** | Integer identifying a WAF firewall version. | [required] |
**filter_exclusion_type** | Option\<**String**> | Filters the results based on this exclusion type. |  |
**filter_name** | Option\<**String**> | Filters the results based on name. |  |
**filter_waf_rules_modsec_rule_id** | Option\<**i32**> | Filters the results based on this ModSecurity rule ID. |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**include** | Option\<**String**> | Include relationships. Optional, comma-separated values. Permitted values: `waf_rules` and `waf_rule_revisions`.  |  |

### Return type

[**crate::models::WafExclusionsResponse**](WafExclusionsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_waf_rule_exclusion

Update a WAF exclusion for a particular firewall version.

```rust
let cfg = &Configuration::default();
let params = UpdateWafRuleExclusionParams {
    // parameters
};
update_waf_rule_exclusion(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**firewall_version_number** | **i32** | Integer identifying a WAF firewall version. | [required] |
**exclusion_number** | **i32** | A numeric ID identifying a WAF exclusion. | [required] |
**waf_exclusion** | Option\<[**WafExclusion**](WafExclusion.md)> |  |  |

### Return type

[**crate::models::WafExclusionResponse**](WafExclusionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

