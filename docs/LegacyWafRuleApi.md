# LegacyWafRuleApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_legacy_waf_firewall_rule_vcl**](LegacyWafRuleApi.md#get_legacy_waf_firewall_rule_vcl) | **GET** /wafs/{firewall_id}/rules/{waf_rule_id}/vcl | Get VCL for a rule associated with a firewall
[**get_legacy_waf_rule**](LegacyWafRuleApi.md#get_legacy_waf_rule) | **GET** /wafs/rules/{waf_rule_id} | Get a rule
[**get_legacy_waf_rule_vcl**](LegacyWafRuleApi.md#get_legacy_waf_rule_vcl) | **GET** /wafs/rules/{waf_rule_id}/vcl | Get VCL for a rule
[**list_legacy_waf_rules**](LegacyWafRuleApi.md#list_legacy_waf_rules) | **GET** /wafs/rules | List rules in the latest configuration set



## get_legacy_waf_firewall_rule_vcl

Get associated VCL for a specific rule associated with a specific firewall.

```rust
let cfg = &Configuration::default();
let params = GetLegacyWafFirewallRuleVclParams {
    // parameters
};
get_legacy_waf_firewall_rule_vcl(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |
**waf_rule_id** | **String** | Alphanumeric string identifying a WAF rule. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_legacy_waf_rule

Get a specific rule.

```rust
let cfg = &Configuration::default();
let params = GetLegacyWafRuleParams {
    // parameters
};
get_legacy_waf_rule(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_rule_id** | **String** | Alphanumeric string identifying a WAF rule. | [required] |
**filter_configuration_set_id** | Option\<**String**> | Optional. Limit rule to a specific configuration set or pass \"all\" to search all configuration sets, including stale ones. |  |
**include** | Option\<**String**> | Include relationships. Optional. Comma separated values. Permitted values: `tags`, `rule_statuses`, `source`, and `vcl`.  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_legacy_waf_rule_vcl

Get associated VCL for a specific rule.

```rust
let cfg = &Configuration::default();
let params = GetLegacyWafRuleVclParams {
    // parameters
};
get_legacy_waf_rule_vcl(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_rule_id** | **String** | Alphanumeric string identifying a WAF rule. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_legacy_waf_rules

List all rules in the latest configuration set.

```rust
let cfg = &Configuration::default();
let params = ListLegacyWafRulesParams {
    // parameters
};
list_legacy_waf_rules(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_rule_id** | Option\<**String**> | Limit the returned rules to a specific rule ID. |  |
**filter_severity** | Option\<**String**> | Limit the returned rules to a specific severity. |  |
**filter_tags_name** | Option\<**String**> | Limit the returned rules to a set linked to a tag by name. |  |
**filter_configuration_set_id** | Option\<**String**> | Optional. Limit rules to specific configuration set or pass \"all\" to search all configuration sets, including stale ones. |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**include** | Option\<**String**> | Include relationships. Optional. Comma separated values. Permitted values: `tags`, `rule_statuses`, and `source`.  |  |

### Return type

[**Vec&lt;serde_json::Value&gt;**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

