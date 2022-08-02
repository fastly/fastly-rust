# WafFirewallVersionsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clone_waf_firewall_version**](WafFirewallVersionsApi.md#clone_waf_firewall_version) | **PUT** /waf/firewalls/{firewall_id}/versions/{firewall_version_number}/clone | Clone a firewall version
[**create_waf_firewall_version**](WafFirewallVersionsApi.md#create_waf_firewall_version) | **POST** /waf/firewalls/{firewall_id}/versions | Create a firewall version
[**deploy_activate_waf_firewall_version**](WafFirewallVersionsApi.md#deploy_activate_waf_firewall_version) | **PUT** /waf/firewalls/{firewall_id}/versions/{firewall_version_number}/activate | Deploy or activate a firewall version
[**get_waf_firewall_version**](WafFirewallVersionsApi.md#get_waf_firewall_version) | **GET** /waf/firewalls/{firewall_id}/versions/{firewall_version_number} | Get a firewall version
[**list_waf_firewall_versions**](WafFirewallVersionsApi.md#list_waf_firewall_versions) | **GET** /waf/firewalls/{firewall_id}/versions | List firewall versions
[**update_waf_firewall_version**](WafFirewallVersionsApi.md#update_waf_firewall_version) | **PATCH** /waf/firewalls/{firewall_id}/versions/{firewall_version_number} | Update a firewall version



## clone_waf_firewall_version

Clone a specific, existing firewall version into a new, draft firewall version.

```rust
let cfg = &Configuration::default();
let params = CloneWafFirewallVersionParams {
    // parameters
};
clone_waf_firewall_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**firewall_version_number** | **i32** | Integer identifying a WAF firewall version. | [required] |

### Return type

[**crate::models::WafFirewallVersionResponse**](WafFirewallVersionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_waf_firewall_version

Create a new, draft firewall version.

```rust
let cfg = &Configuration::default();
let params = CreateWafFirewallVersionParams {
    // parameters
};
create_waf_firewall_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**waf_firewall_version** | Option\<[**WafFirewallVersion**](WafFirewallVersion.md)> |  |  |

### Return type

[**crate::models::WafFirewallVersionResponse**](WafFirewallVersionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## deploy_activate_waf_firewall_version

Deploy or activate a specific firewall version. If a firewall has been disabled, deploying a firewall version will automatically enable the firewall again.

```rust
let cfg = &Configuration::default();
let params = DeployActivateWafFirewallVersionParams {
    // parameters
};
deploy_activate_waf_firewall_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**firewall_version_number** | **i32** | Integer identifying a WAF firewall version. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_waf_firewall_version

Get details about a specific firewall version.

```rust
let cfg = &Configuration::default();
let params = GetWafFirewallVersionParams {
    // parameters
};
get_waf_firewall_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**firewall_version_number** | **i32** | Integer identifying a WAF firewall version. | [required] |
**include** | Option\<**String**> | Include relationships. Optional, comma-separated values. Permitted values: `waf_firewall` and `waf_active_rules`.  |  |

### Return type

[**crate::models::WafFirewallVersionResponse**](WafFirewallVersionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_waf_firewall_versions

Get a list of firewall versions associated with a specific firewall.

```rust
let cfg = &Configuration::default();
let params = ListWafFirewallVersionsParams {
    // parameters
};
list_waf_firewall_versions(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**include** | Option\<**String**> | Include relationships. Optional. |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]

### Return type

[**crate::models::WafFirewallVersionsResponse**](WafFirewallVersionsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_waf_firewall_version

Update a specific firewall version.

```rust
let cfg = &Configuration::default();
let params = UpdateWafFirewallVersionParams {
    // parameters
};
update_waf_firewall_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**firewall_version_number** | **i32** | Integer identifying a WAF firewall version. | [required] |
**waf_firewall_version** | Option\<[**WafFirewallVersion**](WafFirewallVersion.md)> |  |  |

### Return type

[**crate::models::WafFirewallVersionResponse**](WafFirewallVersionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

