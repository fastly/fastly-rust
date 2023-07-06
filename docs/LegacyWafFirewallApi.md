# LegacyWafFirewallApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_legacy_waf_firewall_service**](LegacyWafFirewallApi.md#create_legacy_waf_firewall_service) | **POST** /service/{service_id}/version/{version_id}/wafs | Create a firewall
[**disable_legacy_waf_firewall**](LegacyWafFirewallApi.md#disable_legacy_waf_firewall) | **PATCH** /wafs/{firewall_id}/disable | Disable a firewall
[**enable_legacy_waf_firewall**](LegacyWafFirewallApi.md#enable_legacy_waf_firewall) | **PATCH** /wafs/{firewall_id}/enable | Enable a firewall
[**get_legacy_waf_firewall**](LegacyWafFirewallApi.md#get_legacy_waf_firewall) | **GET** /wafs/{firewall_id} | Get a firewall object
[**get_legacy_waf_firewall_service**](LegacyWafFirewallApi.md#get_legacy_waf_firewall_service) | **GET** /service/{service_id}/version/{version_id}/wafs/{firewall_id} | Get a firewall
[**list_legacy_waf_firewalls**](LegacyWafFirewallApi.md#list_legacy_waf_firewalls) | **GET** /wafs | List active firewalls
[**list_legacy_waf_firewalls_service**](LegacyWafFirewallApi.md#list_legacy_waf_firewalls_service) | **GET** /service/{service_id}/version/{version_id}/wafs | List firewalls
[**update_legacy_waf_firewall_service**](LegacyWafFirewallApi.md#update_legacy_waf_firewall_service) | **PATCH** /service/{service_id}/version/{version_id}/wafs/{firewall_id} | Update a firewall



## create_legacy_waf_firewall_service

Create a firewall object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLegacyWafFirewallServiceParams {
    // parameters
};
create_legacy_waf_firewall_service(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## disable_legacy_waf_firewall

Disable a firewall for a particular service and version. This endpoint is intended to be used in an emergency. Disabling a firewall object for a specific service and version replaces your existing WAF ruleset with an empty ruleset. While disabled, your WAF ruleset will not be applied to your origin traffic. This endpoint is only available to users assigned the role of superuser or above. This is an asynchronous action. To check on the completion of this action, use the related link returned in the response to check on the Update Status of the action.

```rust
let cfg = &Configuration::default();
let params = DisableLegacyWafFirewallParams {
    // parameters
};
disable_legacy_waf_firewall(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## enable_legacy_waf_firewall

Re-enable a firewall object for a particular service and version after it has been disabled. This endpoint is intended to be used in an emergency. When a firewall object is re-enabled, a newly generated WAF ruleset VCL based on the current WAF configuration is used to replace the empty ruleset. This endpoint is only available to users assigned the role of superuser or above. This is an asynchronous action. To check on the completion of this action, use the related link returned in the response to check on the Update Status of the action.

```rust
let cfg = &Configuration::default();
let params = EnableLegacyWafFirewallParams {
    // parameters
};
enable_legacy_waf_firewall(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_legacy_waf_firewall

Get a specific firewall object.

```rust
let cfg = &Configuration::default();
let params = GetLegacyWafFirewallParams {
    // parameters
};
get_legacy_waf_firewall(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |
**include** | Option\<**String**> | Include relationships. Optional, comma separated values. Permitted values: `configuration_set`, `owasp`, `rules`, `rule_statuses`.  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_legacy_waf_firewall_service

Get a specific firewall object.

```rust
let cfg = &Configuration::default();
let params = GetLegacyWafFirewallServiceParams {
    // parameters
};
get_legacy_waf_firewall_service(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_legacy_waf_firewalls

List all active firewall objects.

```rust
let cfg = &Configuration::default();
let params = ListLegacyWafFirewallsParams {
    // parameters
};
list_legacy_waf_firewalls(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_rules_rule_id** | Option\<**String**> | Limit the returned firewalls to a specific rule ID. |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**include** | Option\<**String**> | Include relationships. Optional, comma separated values. Permitted values: `configuration_set`, `owasp`.  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_legacy_waf_firewalls_service

List all firewall objects for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLegacyWafFirewallsServiceParams {
    // parameters
};
list_legacy_waf_firewalls_service(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**include** | Option\<**String**> | Include relationships. Optional, comma separated values. Permitted values: `configuration_set`, `owasp`.  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_legacy_waf_firewall_service

Update a firewall object for a particular service and version. 

```rust
let cfg = &Configuration::default();
let params = UpdateLegacyWafFirewallServiceParams {
    // parameters
};
update_legacy_waf_firewall_service(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
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

