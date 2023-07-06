# LegacyWafRuleStatusApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_waf_firewall_rule_status**](LegacyWafRuleStatusApi.md#get_waf_firewall_rule_status) | **GET** /service/{service_id}/wafs/{firewall_id}/rules/{waf_rule_id}/rule_status | Get the status of a rule on a firewall
[**list_waf_firewall_rule_statuses**](LegacyWafRuleStatusApi.md#list_waf_firewall_rule_statuses) | **GET** /service/{service_id}/wafs/{firewall_id}/rule_statuses | List rule statuses
[**update_waf_firewall_rule_status**](LegacyWafRuleStatusApi.md#update_waf_firewall_rule_status) | **PATCH** /service/{service_id}/wafs/{firewall_id}/rules/{waf_rule_id}/rule_status | Update the status of a rule
[**update_waf_firewall_rule_statuses_tag**](LegacyWafRuleStatusApi.md#update_waf_firewall_rule_statuses_tag) | **POST** /service/{service_id}/wafs/{firewall_id}/rule_statuses | Create or update status of a tagged group of rules



## get_waf_firewall_rule_status

Get a specific rule status object for a particular service, firewall, and rule.

```rust
let cfg = &Configuration::default();
let params = GetWafFirewallRuleStatusParams {
    // parameters
};
get_waf_firewall_rule_status(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
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


## list_waf_firewall_rule_statuses

List all rule statuses for a particular service and firewall.

```rust
let cfg = &Configuration::default();
let params = ListWafFirewallRuleStatusesParams {
    // parameters
};
list_waf_firewall_rule_statuses(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |
**filter_status** | Option\<**String**> | Limit results to rule statuses with the specified status. |  |
**filter_rule_message** | Option\<**String**> | Limit results to rule statuses whose rules have the specified message. |  |
**filter_rule_rule_id** | Option\<**String**> | Limit results to rule statuses whose rules represent the specified ModSecurity rule_id. |  |
**filter_rule_tags** | Option\<**String**> | Limit results to rule statuses whose rules relate to the specified tag IDs. |  |
**filter_rule_tags_name** | Option\<**String**> | Limit results to rule statuses whose rules related to the named tags. |  |
**include** | Option\<**String**> | Include relationships. Optional, comma separated values. Permitted values: `tags`.  |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_waf_firewall_rule_status

Update a rule status for a particular service, firewall, and rule.

```rust
let cfg = &Configuration::default();
let params = UpdateWafFirewallRuleStatusParams {
    // parameters
};
update_waf_firewall_rule_status(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |
**waf_rule_id** | **String** | Alphanumeric string identifying a WAF rule. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_waf_firewall_rule_statuses_tag

Create or update all rule statuses for a particular service and firewall, based on tag name. By default, only rule status for enabled rules (with status log or block) will be updated. To update rule statuses for disabled rules under the specified tag, use the force attribute.

```rust
let cfg = &Configuration::default();
let params = UpdateWafFirewallRuleStatusesTagParams {
    // parameters
};
update_waf_firewall_rule_statuses_tag(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |
**name** | Option\<**String**> | The tag name to use to determine the set of rules to update. For example, OWASP or language-php. |  |
**force** | Option\<**String**> | Whether or not to update rule statuses for disabled rules. Optional. |  |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

