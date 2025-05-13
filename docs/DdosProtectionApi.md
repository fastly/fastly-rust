# DdosProtectionApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**ddos_protection_event_get**](DdosProtectionApi.md#ddos_protection_event_get) | **GET** /ddos-protection/v1/events/{event_id} | Get event by ID
[**ddos_protection_event_list**](DdosProtectionApi.md#ddos_protection_event_list) | **GET** /ddos-protection/v1/events | Get events
[**ddos_protection_event_rule_list**](DdosProtectionApi.md#ddos_protection_event_rule_list) | **GET** /ddos-protection/v1/events/{event_id}/rules | Get all rules for an event
[**ddos_protection_rule_get**](DdosProtectionApi.md#ddos_protection_rule_get) | **GET** /ddos-protection/v1/rules/{rule_id} | Get a rule by ID
[**ddos_protection_traffic_stats_rule_get**](DdosProtectionApi.md#ddos_protection_traffic_stats_rule_get) | **GET** /ddos-protection/v1/events/{event_id}/rules/{rule_id}/traffic-stats | Get traffic stats for a rule



## ddos_protection_event_get

Get event by ID.

```rust
let cfg = &Configuration::default();
let params = DdosProtectionEventGetParams {
    // parameters
};
ddos_protection_event_get(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | Unique ID of the event. | [required] |

### Return type

[**crate::models::DdosProtectionEvent**](DdosProtectionEvent.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## ddos_protection_event_list

Get events.

```rust
let cfg = &Configuration::default();
let params = DdosProtectionEventListParams {
    // parameters
};
ddos_protection_event_list(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option\<**String**> | Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty. |  |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 20]
**service_id** | Option\<**String**> | Filter results based on a service_id. |  |
**from** | Option\<**String**> | Represents the start of a date-time range expressed in RFC 3339 format. |  |
**to** | Option\<**String**> | Represents the end of a date-time range expressed in RFC 3339 format. |  |
**name** | Option\<**String**> |  |  |

### Return type

[**crate::models::InlineResponse2002**](InlineResponse2002.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## ddos_protection_event_rule_list

Get all rules for an event.

```rust
let cfg = &Configuration::default();
let params = DdosProtectionEventRuleListParams {
    // parameters
};
ddos_protection_event_rule_list(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | Unique ID of the event. | [required] |
**cursor** | Option\<**String**> | Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty. |  |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 20]

### Return type

[**crate::models::InlineResponse2003**](InlineResponse2003.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## ddos_protection_rule_get

Get a rule by ID.

```rust
let cfg = &Configuration::default();
let params = DdosProtectionRuleGetParams {
    // parameters
};
ddos_protection_rule_get(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** | Unique ID of the rule. | [required] |

### Return type

[**crate::models::DdosProtectionRule**](DdosProtectionRule.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## ddos_protection_traffic_stats_rule_get

Get traffic stats for a rule.

```rust
let cfg = &Configuration::default();
let params = DdosProtectionTrafficStatsRuleGetParams {
    // parameters
};
ddos_protection_traffic_stats_rule_get(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | Unique ID of the event. | [required] |
**rule_id** | **String** | Unique ID of the rule. | [required] |

### Return type

[**crate::models::DdosProtectionTrafficStats**](DdosProtectionTrafficStats.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

