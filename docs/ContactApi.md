# ContactApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_contacts**](ContactApi.md#create_contacts) | **POST** /customer/{customer_id}/contacts | Add a new customer contact
[**delete_contact**](ContactApi.md#delete_contact) | **DELETE** /customer/{customer_id}/contact/{contact_id} | Delete a contact
[**list_contacts**](ContactApi.md#list_contacts) | **GET** /customer/{customer_id}/contacts | List contacts



## create_contacts

Create a contact.

```rust
let cfg = &Configuration::default();
let params = CreateContactsParams {
    // parameters
};
create_contacts(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |
**user_id** | Option\<**String**> | The alphanumeric string representing the user for this customer contact. |  |
**contact_type** | Option\<**String**> | The type of contact. |  |
**name** | Option\<**String**> | The name of this contact, when user_id is not provided. |  |
**email** | Option\<**String**> | The email of this contact, when a user_id is not provided. |  |
**phone** | Option\<**String**> | The phone number for this contact. Required for primary, technical, and security contact types. |  |
**customer_id2** | Option\<**String**> | The alphanumeric string representing the customer for this customer contact. |  |

### Return type

[**crate::models::ContactResponse**](ContactResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_contact

Delete a contact.

```rust
let cfg = &Configuration::default();
let params = DeleteContactParams {
    // parameters
};
delete_contact(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |
**contact_id** | **String** | An alphanumeric string identifying the customer contact. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_contacts

List all contacts from a specified customer ID.

```rust
let cfg = &Configuration::default();
let params = ListContactsParams {
    // parameters
};
list_contacts(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |

### Return type

[**Vec&lt;crate::models::SchemasContactResponse&gt;**](SchemasContactResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

