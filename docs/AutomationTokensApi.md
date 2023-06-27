# AutomationTokensApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_automation_token**](AutomationTokensApi.md#create_automation_token) | **POST** /automation-tokens | Create Automation Token
[**get_automation_token_id**](AutomationTokensApi.md#get_automation_token_id) | **GET** /automation-tokens/{id} | Retrieve an Automation Token by ID
[**get_automation_tokens_id_services**](AutomationTokensApi.md#get_automation_tokens_id_services) | **GET** /automation-tokens/{id}/services | List Automation Token Services
[**list_automation_tokens**](AutomationTokensApi.md#list_automation_tokens) | **GET** /automation-tokens | List Customer Automation Tokens
[**revoke_automation_token_id**](AutomationTokensApi.md#revoke_automation_token_id) | **DELETE** /automation-tokens/{id} | Revoke an Automation Token by ID



## create_automation_token

Creates a new automation token.

```rust
let cfg = &Configuration::default();
let params = CreateAutomationTokenParams {
    // parameters
};
create_automation_token(cfg, params)
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
- **Accept**: application/vnd.api+json, application/problem+json

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
- **Accept**: application/vnd.api+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_automation_tokens

Lists all automation tokens for a customer.

```rust
let cfg = &Configuration::default();
let params = ListAutomationTokensParams {
    // parameters
};
list_automation_tokens(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option\<**i32**> |  |  |
**page** | Option\<**i32**> |  |  |

### Return type

[**Vec&lt;crate::models::AutomationTokenResponse&gt;**](AutomationTokenResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## revoke_automation_token_id

Revoke an automation token by ID.

```rust
let cfg = &Configuration::default();
let params = RevokeAutomationTokenIdParams {
    // parameters
};
revoke_automation_token_id(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ErrorResponse**](ErrorResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

