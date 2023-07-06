# WafActiveRulesApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_delete_waf_active_rules**](WafActiveRulesApi.md#bulk_delete_waf_active_rules) | **DELETE** /waf/firewalls/{firewall_id}/versions/{version_id}/active-rules | Delete multiple active rules from a WAF
[**bulk_update_waf_active_rules**](WafActiveRulesApi.md#bulk_update_waf_active_rules) | **PATCH** /waf/firewalls/{firewall_id}/versions/{version_id}/active-rules/bulk | Update multiple active rules
[**create_waf_active_rule**](WafActiveRulesApi.md#create_waf_active_rule) | **POST** /waf/firewalls/{firewall_id}/versions/{version_id}/active-rules | Add a rule to a WAF as an active rule
[**create_waf_active_rules_tag**](WafActiveRulesApi.md#create_waf_active_rules_tag) | **POST** /waf/firewalls/{firewall_id}/versions/{version_id}/tags/{waf_tag_name}/active-rules | Create active rules by tag
[**delete_waf_active_rule**](WafActiveRulesApi.md#delete_waf_active_rule) | **DELETE** /waf/firewalls/{firewall_id}/versions/{version_id}/active-rules/{waf_rule_id} | Delete an active rule
[**get_waf_active_rule**](WafActiveRulesApi.md#get_waf_active_rule) | **GET** /waf/firewalls/{firewall_id}/versions/{version_id}/active-rules/{waf_rule_id} | Get an active WAF rule object
[**list_waf_active_rules**](WafActiveRulesApi.md#list_waf_active_rules) | **GET** /waf/firewalls/{firewall_id}/versions/{version_id}/active-rules | List active rules on a WAF
[**update_waf_active_rule**](WafActiveRulesApi.md#update_waf_active_rule) | **PATCH** /waf/firewalls/{firewall_id}/versions/{version_id}/active-rules/{waf_rule_id} | Update an active rule



## bulk_delete_waf_active_rules

Delete many active rules on a particular firewall version using the active rule ID. Limited to 500 rules per request.

```rust
let cfg = &Configuration::default();
let params = BulkDeleteWafActiveRulesParams {
    // parameters
};
bulk_delete_waf_active_rules(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json; ext=bulk
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## bulk_update_waf_active_rules

Bulk update all active rules on a [firewall version](https://developer.fastly.com/reference/api/waf/firewall-version/). This endpoint will not add new active rules, only update existing active rules.

```rust
let cfg = &Configuration::default();
let params = BulkUpdateWafActiveRulesParams {
    // parameters
};
bulk_update_waf_active_rules(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**body** | Option\<**crate::models::WafActiveRuleData**> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_waf_active_rule

Create an active rule for a particular firewall version.

```rust
let cfg = &Configuration::default();
let params = CreateWafActiveRuleParams {
    // parameters
};
create_waf_active_rule(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**waf_active_rule** | Option\<[**WafActiveRule**](WafActiveRule.md)> |  |  |

### Return type

[**crate::models::WafActiveRuleCreationResponse**](WafActiveRuleCreationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json, application/vnd.api+json; ext=bulk
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_waf_active_rules_tag

Create active rules by tag. This endpoint will create active rules using the latest revision available for each rule.

```rust
let cfg = &Configuration::default();
let params = CreateWafActiveRulesTagParams {
    // parameters
};
create_waf_active_rules_tag(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**waf_tag_name** | **String** | Name of the tag. | [required] |
**waf_active_rule** | Option\<[**WafActiveRule**](WafActiveRule.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_waf_active_rule

Delete an active rule for a particular firewall version.

```rust
let cfg = &Configuration::default();
let params = DeleteWafActiveRuleParams {
    // parameters
};
delete_waf_active_rule(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**waf_rule_id** | **String** | Alphanumeric string identifying a WAF rule. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_waf_active_rule

Get a specific active rule object. Includes details of the rule revision associated with the active rule object by default.

```rust
let cfg = &Configuration::default();
let params = GetWafActiveRuleParams {
    // parameters
};
get_waf_active_rule(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**waf_rule_id** | **String** | Alphanumeric string identifying a WAF rule. | [required] |
**include** | Option\<**String**> | Include relationships. Optional, comma-separated values. Permitted values: `waf_rule_revision` and `waf_firewall_version`.  |  |

### Return type

[**crate::models::WafActiveRuleResponse**](WafActiveRuleResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_waf_active_rules

List all active rules for a particular firewall version.

```rust
let cfg = &Configuration::default();
let params = ListWafActiveRulesParams {
    // parameters
};
list_waf_active_rules(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**filter_status** | Option\<**String**> | Limit results to active rules with the specified status. |  |
**filter_waf_rule_revision_message** | Option\<**String**> | Limit results to active rules with the specified message. |  |
**filter_waf_rule_revision_modsec_rule_id** | Option\<**String**> | Limit results to active rules that represent the specified ModSecurity modsec_rule_id. |  |
**filter_outdated** | Option\<**String**> | Limit results to active rules referencing an outdated rule revision. |  |
**include** | Option\<**String**> | Include relationships. Optional, comma-separated values. Permitted values: `waf_rule_revision` and `waf_firewall_version`.  |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]

### Return type

[**crate::models::WafActiveRulesResponse**](WafActiveRulesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_waf_active_rule

Update an active rule's status for a particular firewall version.

```rust
let cfg = &Configuration::default();
let params = UpdateWafActiveRuleParams {
    // parameters
};
update_waf_active_rule(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**waf_rule_id** | **String** | Alphanumeric string identifying a WAF rule. | [required] |
**waf_active_rule** | Option\<[**WafActiveRule**](WafActiveRule.md)> |  |  |

### Return type

[**crate::models::WafActiveRuleResponse**](WafActiveRuleResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

