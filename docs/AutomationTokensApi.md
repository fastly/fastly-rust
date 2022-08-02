# AutomationTokensApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_automation_token_id**](AutomationTokensApi.md#delete_automation_token_id) | **DELETE** /automation-tokens/{id} | Delete an Automation Token by ID
[**get_automation_token**](AutomationTokensApi.md#get_automation_token) | **GET** /automation-tokens | List Automation Tokens for a customer
[**get_automation_token_id**](AutomationTokensApi.md#get_automation_token_id) | **GET** /automation-tokens/{id} | Retrieve an Automation Token by ID
[**get_automation_tokens_id_services**](AutomationTokensApi.md#get_automation_tokens_id_services) | **GET** /automation-tokens/{id}/services | List Automation Token Services
[**post_automation_token**](AutomationTokensApi.md#post_automation_token) | **POST** /automation-tokens | Create Automation Token



## delete_automation_token_id

Delete an automation token by ID.

```rust
let cfg = &Configuration::default();
let params = DeleteAutomationTokenIdParams {
    // parameters
};
delete_automation_token_id(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_automation_token

Lists all automation tokens for a customer.

```rust
let cfg = &Configuration::default();
let params = GetAutomationTokenParams {
    // parameters
};
get_automation_token(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option\<**i32**> |  |  |
**page** | Option\<**i32**> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_automation_token_id

Retrieves an automation token by ID.

```rust
let cfg = &Configuration::default();
let params = GetAutomationTokenIdParams {
    // parameters
};
get_automation_token_id(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::AutomationTokenResponse**](AutomationTokenResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_automation_tokens_id_services

List of services associated with the automation token.

```rust
let cfg = &Configuration::default();
let params = GetAutomationTokensIdServicesParams {
    // parameters
};
get_automation_tokens_id_services(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**per_page** | Option\<**i32**> |  |  |
**page** | Option\<**i32**> |  |  |

### Return type

[**crate::models::InlineResponse2001**](InlineResponse2001.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## post_automation_token

Creates a new automation token

```rust
let cfg = &Configuration::default();
let params = PostAutomationTokenParams {
    // parameters
};
post_automation_token(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**automation_token_create_request** | Option\<[**AutomationTokenCreateRequest**](AutomationTokenCreateRequest.md)> |  |  |

### Return type

[**crate::models::AutomationTokenCreateResponse**](AutomationTokenCreateResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

