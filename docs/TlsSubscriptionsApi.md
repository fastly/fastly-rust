# TlsSubscriptionsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_globalsign_email_challenge**](TlsSubscriptionsApi.md#create_globalsign_email_challenge) | **POST** /tls/subscriptions/{tls_subscription_id}/authorizations/{tls_authorization_id}/globalsign_email_challenges | Creates a GlobalSign email challenge.
[**create_tls_sub**](TlsSubscriptionsApi.md#create_tls_sub) | **POST** /tls/subscriptions | Create a TLS subscription
[**delete_globalsign_email_challenge**](TlsSubscriptionsApi.md#delete_globalsign_email_challenge) | **DELETE** /tls/subscriptions/{tls_subscription_id}/authorizations/{tls_authorization_id}/globalsign_email_challenges/{globalsign_email_challenge_id} | Delete a GlobalSign email challenge
[**delete_tls_sub**](TlsSubscriptionsApi.md#delete_tls_sub) | **DELETE** /tls/subscriptions/{tls_subscription_id} | Delete a TLS subscription
[**get_tls_sub**](TlsSubscriptionsApi.md#get_tls_sub) | **GET** /tls/subscriptions/{tls_subscription_id} | Get a TLS subscription
[**list_tls_subs**](TlsSubscriptionsApi.md#list_tls_subs) | **GET** /tls/subscriptions | List TLS subscriptions
[**patch_tls_sub**](TlsSubscriptionsApi.md#patch_tls_sub) | **PATCH** /tls/subscriptions/{tls_subscription_id} | Update a TLS subscription



## create_globalsign_email_challenge

Creates an email challenge for a domain on a GlobalSign subscription. An email challenge will generate an email that can be used to validate domain ownership. If this challenge is created, then the domain can only be validated using email for the given subscription. 

```rust
let cfg = &Configuration::default();
let params = CreateGlobalsignEmailChallengeParams {
    // parameters
};
create_globalsign_email_challenge(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_subscription_id** | **String** | Alphanumeric string identifying a TLS subscription. | [required] |
**tls_authorization_id** | **String** | Alphanumeric string identifying a TLS subscription. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_tls_sub

Create a new TLS subscription. This response includes a list of possible challenges to verify domain ownership.

```rust
let cfg = &Configuration::default();
let params = CreateTlsSubParams {
    // parameters
};
create_tls_sub(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force** | Option\<**bool**> | A flag that allows you to edit and delete a subscription with active domains. Valid to use on PATCH and DELETE actions. As a warning, removing an active domain from a subscription or forcing the deletion of a subscription may result in breaking TLS termination to that domain.  |  |
**tls_subscription** | Option\<[**TlsSubscription**](TlsSubscription.md)> |  |  |

### Return type

[**crate::models::TlsSubscriptionResponse**](TlsSubscriptionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_globalsign_email_challenge

Deletes a GlobalSign email challenge. After a GlobalSign email challenge is deleted, the domain can use HTTP and DNS validation methods again.

```rust
let cfg = &Configuration::default();
let params = DeleteGlobalsignEmailChallengeParams {
    // parameters
};
delete_globalsign_email_challenge(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_subscription_id** | **String** | Alphanumeric string identifying a TLS subscription. | [required] |
**globalsign_email_challenge_id** | **String** | Alphanumeric string identifying a GlobalSign email challenge. | [required] |
**tls_authorization_id** | **String** | Alphanumeric string identifying a TLS subscription. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_tls_sub

Destroy a TLS subscription. A subscription cannot be destroyed if there are domains in the TLS enabled state.

```rust
let cfg = &Configuration::default();
let params = DeleteTlsSubParams {
    // parameters
};
delete_tls_sub(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_subscription_id** | **String** | Alphanumeric string identifying a TLS subscription. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tls_sub

Show a TLS subscription.

```rust
let cfg = &Configuration::default();
let params = GetTlsSubParams {
    // parameters
};
get_tls_sub(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_subscription_id** | **String** | Alphanumeric string identifying a TLS subscription. | [required] |
**include** | Option\<**String**> | Include related objects. Optional, comma-separated values. Permitted values: `tls_authorizations` and `tls_authorizations.globalsign_email_challenge`.  |  |

### Return type

[**crate::models::TlsSubscriptionResponse**](TlsSubscriptionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tls_subs

List all TLS subscriptions.

```rust
let cfg = &Configuration::default();
let params = ListTlsSubsParams {
    // parameters
};
list_tls_subs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_state** | Option\<**String**> | Limit the returned subscriptions by state. Valid values are `pending`, `processing`, `issued`, `renewing`, and `failed`. Accepts parameters: `not` (e.g., `filter[state][not]=renewing`).  |  |
**filter_tls_domains_id** | Option\<**String**> | Limit the returned subscriptions to those that include the specific domain. |  |
**filter_has_active_order** | Option\<**bool**> | Limit the returned subscriptions to those that have currently active orders. Permitted values: `true`.  |  |
**include** | Option\<**String**> | Include related objects. Optional, comma-separated values. Permitted values: `tls_authorizations` and `tls_authorizations.globalsign_email_challenge`.  |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**sort** | Option\<**String**> | The order in which to list the results by creation date. |  |[default to created_at]

### Return type

[**crate::models::TlsSubscriptionsResponse**](TlsSubscriptionsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## patch_tls_sub

Change the TLS domains or common name associated with this subscription, update the TLS configuration for this set of domains, or retry a subscription with state `failed` by setting the state to `retry`.

```rust
let cfg = &Configuration::default();
let params = PatchTlsSubParams {
    // parameters
};
patch_tls_sub(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_subscription_id** | **String** | Alphanumeric string identifying a TLS subscription. | [required] |
**force** | Option\<**bool**> | A flag that allows you to edit and delete a subscription with active domains. Valid to use on PATCH and DELETE actions. As a warning, removing an active domain from a subscription or forcing the deletion of a subscription may result in breaking TLS termination to that domain.  |  |
**tls_subscription** | Option\<[**TlsSubscription**](TlsSubscription.md)> |  |  |

### Return type

[**crate::models::TlsSubscriptionResponse**](TlsSubscriptionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

