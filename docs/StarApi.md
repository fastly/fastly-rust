# StarApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_service_star**](StarApi.md#create_service_star) | **POST** /stars | Create a star
[**delete_service_star**](StarApi.md#delete_service_star) | **DELETE** /stars/{star_id} | Delete a star
[**get_service_star**](StarApi.md#get_service_star) | **GET** /stars/{star_id} | Get a star
[**list_service_stars**](StarApi.md#list_service_stars) | **GET** /stars | List stars



## create_service_star

Create star.

```rust
let cfg = &Configuration::default();
let params = CreateServiceStarParams {
    // parameters
};
create_service_star(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**star** | Option\<[**Star**](Star.md)> |  |  |

### Return type

[**crate::models::StarResponse**](StarResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_service_star

Delete star.

```rust
let cfg = &Configuration::default();
let params = DeleteServiceStarParams {
    // parameters
};
delete_service_star(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**star_id** | **String** | Alphanumeric string identifying a star. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_service_star

Show star.

```rust
let cfg = &Configuration::default();
let params = GetServiceStarParams {
    // parameters
};
get_service_star(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**star_id** | **String** | Alphanumeric string identifying a star. | [required] |

### Return type

[**crate::models::StarResponse**](StarResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_service_stars

List stars.

```rust
let cfg = &Configuration::default();
let params = ListServiceStarsParams {
    // parameters
};
list_service_stars(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Pagination**](Pagination.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

