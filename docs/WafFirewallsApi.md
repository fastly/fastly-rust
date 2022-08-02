# WafFirewallsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_waf_firewall**](WafFirewallsApi.md#create_waf_firewall) | **POST** /waf/firewalls | Create a firewall
[**delete_waf_firewall**](WafFirewallsApi.md#delete_waf_firewall) | **DELETE** /waf/firewalls/{firewall_id} | Delete a firewall
[**get_waf_firewall**](WafFirewallsApi.md#get_waf_firewall) | **GET** /waf/firewalls/{firewall_id} | Get a firewall
[**list_waf_firewalls**](WafFirewallsApi.md#list_waf_firewalls) | **GET** /waf/firewalls | List firewalls
[**update_waf_firewall**](WafFirewallsApi.md#update_waf_firewall) | **PATCH** /waf/firewalls/{firewall_id} | Update a firewall



## create_waf_firewall

Create a firewall object for a particular service and service version using a defined `prefetch_condition` and `response`. If the `prefetch_condition` or the `response` is missing from the request body, Fastly will generate a default object on your service. 

```rust
let cfg = &Configuration::default();
let params = CreateWafFirewallParams {
    // parameters
};
create_waf_firewall(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_firewall** | Option\<[**WafFirewall**](WafFirewall.md)> |  |  |

### Return type

[**crate::models::WafFirewallResponse**](WafFirewallResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_waf_firewall

Delete the firewall object for a particular service and service version. 

```rust
let cfg = &Configuration::default();
let params = DeleteWafFirewallParams {
    // parameters
};
delete_waf_firewall(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**waf_firewall** | Option\<[**WafFirewall**](WafFirewall.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_waf_firewall

Get a specific firewall object.

```rust
let cfg = &Configuration::default();
let params = GetWafFirewallParams {
    // parameters
};
get_waf_firewall(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**filter_service_version_number** | Option\<**String**> | Limit the results returned to a specific service version. |  |
**include** | Option\<**String**> | Include related objects. Optional. |  |[default to waf_firewall_versions]

### Return type

[**crate::models::WafFirewallResponse**](WafFirewallResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_waf_firewalls

List all firewall objects.

```rust
let cfg = &Configuration::default();
let params = ListWafFirewallsParams {
    // parameters
};
list_waf_firewalls(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**filter_service_id** | Option\<**String**> | Limit the results returned to a specific service. |  |
**filter_service_version_number** | Option\<**String**> | Limit the results returned to a specific service version. |  |
**include** | Option\<**String**> | Include related objects. Optional. |  |[default to waf_firewall_versions]

### Return type

[**crate::models::WafFirewallsResponse**](WafFirewallsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_waf_firewall

Update a firewall object for a particular service and service version. Specifying a `service_version_number` is required. 

```rust
let cfg = &Configuration::default();
let params = UpdateWafFirewallParams {
    // parameters
};
update_waf_firewall(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a WAF Firewall. | [required] |
**waf_firewall** | Option\<[**WafFirewall**](WafFirewall.md)> |  |  |

### Return type

[**crate::models::WafFirewallResponse**](WafFirewallResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

