# CustomerApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_customer**](CustomerApi.md#delete_customer) | **DELETE** /customer/{customer_id} | Delete a customer
[**get_customer**](CustomerApi.md#get_customer) | **GET** /customer/{customer_id} | Get a customer
[**get_logged_in_customer**](CustomerApi.md#get_logged_in_customer) | **GET** /current_customer | Get the logged in customer
[**list_users**](CustomerApi.md#list_users) | **GET** /customer/{customer_id}/users | List users
[**update_customer**](CustomerApi.md#update_customer) | **PUT** /customer/{customer_id} | Update a customer



## delete_customer

Delete a customer.

```rust
let cfg = &Configuration::default();
let params = DeleteCustomerParams {
    // parameters
};
delete_customer(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_customer

Get a specific customer.

```rust
let cfg = &Configuration::default();
let params = GetCustomerParams {
    // parameters
};
get_customer(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |

### Return type

[**crate::models::CustomerResponse**](CustomerResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_logged_in_customer

Get the logged in customer.

```rust
let cfg = &Configuration::default();
let params = GetLoggedInCustomerParams {
    // parameters
};
get_logged_in_customer(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CustomerResponse**](CustomerResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_users

List all users from a specified customer id.

```rust
let cfg = &Configuration::default();
let params = ListUsersParams {
    // parameters
};
list_users(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |

### Return type

[**Vec&lt;crate::models::SchemasUserResponse&gt;**](SchemasUserResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_customer

Update a customer.

```rust
let cfg = &Configuration::default();
let params = UpdateCustomerParams {
    // parameters
};
update_customer(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |
**billing_contact_id** | Option\<**String**> | The alphanumeric string representing the primary billing contact. |  |
**billing_network_type** | Option\<**String**> | Customer's current network revenue type. |  |
**billing_ref** | Option\<**String**> | Used for adding purchased orders to customer's account. |  |
**can_configure_wordpress** | Option\<**bool**> | Whether this customer can view or edit wordpress. |  |
**can_reset_passwords** | Option\<**bool**> | Whether this customer can reset passwords. |  |
**can_upload_vcl** | Option\<**bool**> | Whether this customer can upload VCL. |  |
**force_2fa** | Option\<**bool**> | Specifies whether 2FA is forced or not forced on the customer account. Logs out non-2FA users once 2FA is force enabled. |  |
**force_sso** | Option\<**bool**> | Specifies whether SSO is forced or not forced on the customer account. |  |
**has_account_panel** | Option\<**bool**> | Specifies whether the account has access or does not have access to the account panel. |  |
**has_improved_events** | Option\<**bool**> | Specifies whether the account has access or does not have access to the improved events. |  |
**has_improved_ssl_config** | Option\<**bool**> | Whether this customer can view or edit the SSL config. |  |
**has_openstack_logging** | Option\<**bool**> | Specifies whether the account has enabled or not enabled openstack logging. |  |
**has_pci** | Option\<**bool**> | Specifies whether the account can edit PCI for a service. |  |
**has_pci_passwords** | Option\<**bool**> | Specifies whether PCI passwords are required for the account. |  |
**ip_whitelist** | Option\<**String**> | The range of IP addresses authorized to access the customer account. |  |
**legal_contact_id** | Option\<**String**> | The alphanumeric string identifying the account's legal contact. |  |
**name** | Option\<**String**> | The name of the customer, generally the company name. |  |
**owner_id** | Option\<**String**> | The alphanumeric string identifying the account owner. |  |
**phone_number** | Option\<**String**> | The phone number associated with the account. |  |
**postal_address** | Option\<**String**> | The postal address associated with the account. |  |
**pricing_plan** | Option\<**String**> | The pricing plan this customer is under. |  |
**pricing_plan_id** | Option\<**String**> | The alphanumeric string identifying the pricing plan. |  |
**security_contact_id** | Option\<**String**> | The alphanumeric string identifying the account's security contact. |  |
**technical_contact_id** | Option\<**String**> | The alphanumeric string identifying the account's technical contact. |  |

### Return type

[**crate::models::CustomerResponse**](CustomerResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

