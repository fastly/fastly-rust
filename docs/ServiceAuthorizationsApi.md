# ServiceAuthorizationsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_service_authorization**](ServiceAuthorizationsApi.md#create_service_authorization) | **POST** /service-authorizations | Create service authorization
[**delete_service_authorization**](ServiceAuthorizationsApi.md#delete_service_authorization) | **DELETE** /service-authorizations/{service_authorization_id} | Delete service authorization
[**delete_service_authorization2**](ServiceAuthorizationsApi.md#delete_service_authorization2) | **DELETE** /service-authorizations | Delete service authorizations
[**list_service_authorization**](ServiceAuthorizationsApi.md#list_service_authorization) | **GET** /service-authorizations | List service authorizations
[**show_service_authorization**](ServiceAuthorizationsApi.md#show_service_authorization) | **GET** /service-authorizations/{service_authorization_id} | Show service authorization
[**update_service_authorization**](ServiceAuthorizationsApi.md#update_service_authorization) | **PATCH** /service-authorizations/{service_authorization_id} | Update service authorization
[**update_service_authorization2**](ServiceAuthorizationsApi.md#update_service_authorization2) | **PATCH** /service-authorizations | Update service authorizations



## create_service_authorization

Create service authorization.

```rust
let cfg = &Configuration::default();
let params = CreateServiceAuthorizationParams {
    // parameters
};
create_service_authorization(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_authorization** | Option\<[**ServiceAuthorization**](ServiceAuthorization.md)> |  |  |

### Return type

[**crate::models::ServiceAuthorizationResponse**](ServiceAuthorizationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_service_authorization

Delete service authorization.

```rust
let cfg = &Configuration::default();
let params = DeleteServiceAuthorizationParams {
    // parameters
};
delete_service_authorization(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_authorization_id** | **String** | Alphanumeric string identifying a service authorization. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_service_authorization2

Delete service authorizations.

```rust
let cfg = &Configuration::default();
let params = DeleteServiceAuthorization2Params {
    // parameters
};
delete_service_authorization2(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**crate::models::InlineResponse2007**](InlineResponse2007.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json; ext=bulk
- **Accept**: application/vnd.api+json; ext=bulk

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_service_authorization

List service authorizations.

```rust
let cfg = &Configuration::default();
let params = ListServiceAuthorizationParams {
    // parameters
};
list_service_authorization(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]

### Return type

[**crate::models::ServiceAuthorizationsResponse**](ServiceAuthorizationsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## show_service_authorization

Show service authorization.

```rust
let cfg = &Configuration::default();
let params = ShowServiceAuthorizationParams {
    // parameters
};
show_service_authorization(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_authorization_id** | **String** | Alphanumeric string identifying a service authorization. | [required] |

### Return type

[**crate::models::ServiceAuthorizationResponse**](ServiceAuthorizationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_service_authorization

Update service authorization.

```rust
let cfg = &Configuration::default();
let params = UpdateServiceAuthorizationParams {
    // parameters
};
update_service_authorization(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_authorization_id** | **String** | Alphanumeric string identifying a service authorization. | [required] |
**service_authorization** | Option\<[**ServiceAuthorization**](ServiceAuthorization.md)> |  |  |

### Return type

[**crate::models::ServiceAuthorizationResponse**](ServiceAuthorizationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_service_authorization2

Update service authorizations.

```rust
let cfg = &Configuration::default();
let params = UpdateServiceAuthorization2Params {
    // parameters
};
update_service_authorization2(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**crate::models::ServiceAuthorizationsResponse**](ServiceAuthorizationsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json; ext=bulk
- **Accept**: application/vnd.api+json; ext=bulk

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

