# CustomerAddressesApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**create_customer_address**](CustomerAddressesApi.md#create_customer_address) | **POST** /billing/v3/customer-addresses | Creates an address associated with a customer account.
[**list_customer_addresses**](CustomerAddressesApi.md#list_customer_addresses) | **GET** /billing/v3/customer-addresses | Return the list of addresses associated with a customer account.
[**update_customer_address**](CustomerAddressesApi.md#update_customer_address) | **PUT** /billing/v3/customer-addresses/{type} | Updates an address associated with a customer account.



## create_customer_address

Creates an address associated with a customer account.

```rust
let cfg = &Configuration::default();
let params = CreateCustomerAddressParams {
    // parameters
};
create_customer_address(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_address** | [**CustomerAddress**](CustomerAddress.md) |  | [required] |

### Return type

[**crate::models::InlineResponse201**](InlineResponse201.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_customer_addresses

Return the list of addresses associated with a customer account.

```rust
let cfg = &Configuration::default();
let params = ListCustomerAddressesParams {
    // parameters
};
list_customer_addresses(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListCustomerAddressesResponse**](ListCustomerAddressesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_customer_address

Updates an address associated with a customer account.

```rust
let cfg = &Configuration::default();
let params = UpdateCustomerAddressParams {
    // parameters
};
update_customer_address(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** | Alphanumeric type of the address being modified. | [required] |
**customer_address** | [**CustomerAddress**](CustomerAddress.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

