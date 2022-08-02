# BillingAddressApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_billing_addr**](BillingAddressApi.md#add_billing_addr) | **POST** /customer/{customer_id}/billing_address | Add a billing address to a customer
[**delete_billing_addr**](BillingAddressApi.md#delete_billing_addr) | **DELETE** /customer/{customer_id}/billing_address | Delete a billing address
[**get_billing_addr**](BillingAddressApi.md#get_billing_addr) | **GET** /customer/{customer_id}/billing_address | Get a billing address
[**update_billing_addr**](BillingAddressApi.md#update_billing_addr) | **PATCH** /customer/{customer_id}/billing_address | Update a billing address



## add_billing_addr

Add a billing address to a customer.

```rust
let cfg = &Configuration::default();
let params = AddBillingAddrParams {
    // parameters
};
add_billing_addr(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |
**billing_address_request** | Option\<[**BillingAddressRequest**](BillingAddressRequest.md)> | Billing address |  |

### Return type

[**crate::models::BillingAddressResponse**](BillingAddressResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_billing_addr

Delete a customer's billing address.

```rust
let cfg = &Configuration::default();
let params = DeleteBillingAddrParams {
    // parameters
};
delete_billing_addr(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_billing_addr

Get a customer's billing address.

```rust
let cfg = &Configuration::default();
let params = GetBillingAddrParams {
    // parameters
};
get_billing_addr(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |

### Return type

[**crate::models::BillingAddressResponse**](BillingAddressResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_billing_addr

Update a customer's billing address. You may update only part of the customer's billing address.

```rust
let cfg = &Configuration::default();
let params = UpdateBillingAddrParams {
    // parameters
};
update_billing_addr(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |
**update_billing_address_request** | Option\<[**UpdateBillingAddressRequest**](UpdateBillingAddressRequest.md)> | One or more billing address attributes |  |

### Return type

[**crate::models::BillingAddressResponse**](BillingAddressResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

