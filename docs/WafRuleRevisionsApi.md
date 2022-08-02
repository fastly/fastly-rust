# WafRuleRevisionsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_waf_rule_revision**](WafRuleRevisionsApi.md#get_waf_rule_revision) | **GET** /waf/rules/{waf_rule_id}/revisions/{waf_rule_revision_number} | Get a revision of a rule
[**list_waf_rule_revisions**](WafRuleRevisionsApi.md#list_waf_rule_revisions) | **GET** /waf/rules/{waf_rule_id}/revisions | List revisions for a rule



## get_waf_rule_revision

Get a specific rule revision.

```rust
let cfg = &Configuration::default();
let params = GetWafRuleRevisionParams {
    // parameters
};
get_waf_rule_revision(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_rule_id** | **String** | Alphanumeric string identifying a WAF rule. | [required] |
**waf_rule_revision_number** | **i32** | Revision number. | [required] |
**include** | Option\<**String**> | Include relationships. Optional, comma-separated values. Permitted values: `waf_rule`, `vcl`, and `source`. The `vcl` and `source` relationships show the WAF VCL and corresponding ModSecurity source. These fields are blank unless the relationship is included.  |  |

### Return type

[**crate::models::WafRuleRevisionResponse**](WafRuleRevisionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_waf_rule_revisions

List all revisions for a specific rule. The `rule_id` provided can be the ModSecurity Rule ID or the Fastly generated rule ID.

```rust
let cfg = &Configuration::default();
let params = ListWafRuleRevisionsParams {
    // parameters
};
list_waf_rule_revisions(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_rule_id** | **String** | Alphanumeric string identifying a WAF rule. | [required] |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**include** | Option\<**String**> | Include relationships. Optional. |  |[default to waf_rule]

### Return type

[**crate::models::WafRuleRevisionsResponse**](WafRuleRevisionsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

