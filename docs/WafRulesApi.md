# WafRulesApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_waf_rule**](WafRulesApi.md#get_waf_rule) | **GET** /waf/rules/{waf_rule_id} | Get a rule
[**list_waf_rules**](WafRulesApi.md#list_waf_rules) | **GET** /waf/rules | List available WAF rules



## get_waf_rule

Get a specific rule. The `id` provided can be the ModSecurity Rule ID or the Fastly generated rule ID.

```rust
let cfg = &Configuration::default();
let params = GetWafRuleParams {
    // parameters
};
get_waf_rule(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_rule_id** | **String** | Alphanumeric string identifying a WAF rule. | [required] |
**include** | Option\<**String**> | Include relationships. Optional, comma-separated values. Permitted values: `waf_tags` and `waf_rule_revisions`.  |  |

### Return type

[**crate::models::WafRuleResponse**](WafRuleResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_waf_rules

List all available WAF rules.

```rust
let cfg = &Configuration::default();
let params = ListWafRulesParams {
    // parameters
};
list_waf_rules(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_modsec_rule_id** | Option\<**String**> | Limit the returned rules to a specific ModSecurity rule ID. |  |
**filter_waf_tags_name** | Option\<**String**> | Limit the returned rules to a set linked to a tag by name. |  |
**filter_waf_rule_revisions_source** | Option\<**String**> | Limit the returned rules to a set linked to a source. |  |
**filter_waf_firewall_id_not_match** | Option\<**String**> | Limit the returned rules to a set not included in the active firewall version for a firewall. |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**include** | Option\<**String**> | Include relationships. Optional, comma-separated values. Permitted values: `waf_tags` and `waf_rule_revisions`.  |  |

### Return type

[**crate::models::WafRulesResponse**](WafRulesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

