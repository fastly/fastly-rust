# ApisecurityOperationsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**api_security_bulk_add_tags_to_operations**](ApisecurityOperationsApi.md#api_security_bulk_add_tags_to_operations) | **POST** /api-security/v1/services/{service_id}/operations-bulk-tags | Bulk add tags to operations
[**api_security_bulk_create_operations**](ApisecurityOperationsApi.md#api_security_bulk_create_operations) | **POST** /api-security/v1/services/{service_id}/operations-bulk | Bulk create operations
[**api_security_bulk_delete_operations**](ApisecurityOperationsApi.md#api_security_bulk_delete_operations) | **DELETE** /api-security/v1/services/{service_id}/operations-bulk | Bulk delete operations
[**api_security_create_operation**](ApisecurityOperationsApi.md#api_security_create_operation) | **POST** /api-security/v1/services/{service_id}/operations | Create operation
[**api_security_create_operation_tag**](ApisecurityOperationsApi.md#api_security_create_operation_tag) | **POST** /api-security/v1/services/{service_id}/tags | Create operation tag
[**api_security_delete_operation**](ApisecurityOperationsApi.md#api_security_delete_operation) | **DELETE** /api-security/v1/services/{service_id}/operations/{operation_id} | Delete operation
[**api_security_delete_operation_tag**](ApisecurityOperationsApi.md#api_security_delete_operation_tag) | **DELETE** /api-security/v1/services/{service_id}/tags/{tag_id} | Delete operation tag
[**api_security_get_operation**](ApisecurityOperationsApi.md#api_security_get_operation) | **GET** /api-security/v1/services/{service_id}/operations/{operation_id} | Retrieve operation
[**api_security_get_operation_tag**](ApisecurityOperationsApi.md#api_security_get_operation_tag) | **GET** /api-security/v1/services/{service_id}/tags/{tag_id} | Retrieve operation tag
[**api_security_list_discovered_operations**](ApisecurityOperationsApi.md#api_security_list_discovered_operations) | **GET** /api-security/v1/services/{service_id}/discovered-operations | List discovered operations
[**api_security_list_operation_tags**](ApisecurityOperationsApi.md#api_security_list_operation_tags) | **GET** /api-security/v1/services/{service_id}/tags | List operation tags
[**api_security_list_operations**](ApisecurityOperationsApi.md#api_security_list_operations) | **GET** /api-security/v1/services/{service_id}/operations | List operations
[**api_security_update_operation**](ApisecurityOperationsApi.md#api_security_update_operation) | **PATCH** /api-security/v1/services/{service_id}/operations/{operation_id} | Update operation
[**api_security_update_operation_tag**](ApisecurityOperationsApi.md#api_security_update_operation_tag) | **PATCH** /api-security/v1/services/{service_id}/tags/{tag_id} | Update operation tag



## api_security_bulk_add_tags_to_operations

Add tags to multiple operations in a single request.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityBulkAddTagsToOperationsParams {
    // parameters
};
api_security_bulk_add_tags_to_operations(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**operation_bulk_add_tags** | Option\<[**OperationBulkAddTags**](OperationBulkAddTags.md)> |  |  |

### Return type

[**crate::models::InlineResponse2071**](InlineResponse2071.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_bulk_create_operations

Create multiple operations associated with a specific service in a single request.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityBulkCreateOperationsParams {
    // parameters
};
api_security_bulk_create_operations(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**operation_bulk_create** | Option\<[**OperationBulkCreate**](OperationBulkCreate.md)> |  |  |

### Return type

[**crate::models::InlineResponse207**](InlineResponse207.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_bulk_delete_operations

Delete multiple operations in a single request.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityBulkDeleteOperationsParams {
    // parameters
};
api_security_bulk_delete_operations(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**operation_bulk_delete** | Option\<[**OperationBulkDelete**](OperationBulkDelete.md)> |  |  |

### Return type

[**crate::models::InlineResponse2071**](InlineResponse2071.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_create_operation

Create a new operation associated with a specific service.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityCreateOperationParams {
    // parameters
};
api_security_create_operation(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**operation_create** | Option\<[**OperationCreate**](OperationCreate.md)> |  |  |

### Return type

[**crate::models::OperationGet**](OperationGet.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_create_operation_tag

Create a new operation tag associated with a specific service.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityCreateOperationTagParams {
    // parameters
};
api_security_create_operation_tag(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**tag_create** | Option\<[**TagCreate**](TagCreate.md)> |  |  |

### Return type

[**crate::models::TagGet**](TagGet.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_delete_operation

Delete an existing operation associated with a specific service.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityDeleteOperationParams {
    // parameters
};
api_security_delete_operation(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**operation_id** | **String** | The unique identifier of the operation. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_delete_operation_tag

Delete an existing operation tag.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityDeleteOperationTagParams {
    // parameters
};
api_security_delete_operation_tag(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**tag_id** | **String** | The unique identifier of the operation tag. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_get_operation

Get a specific operation associated with a service.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityGetOperationParams {
    // parameters
};
api_security_get_operation(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**operation_id** | **String** | The unique identifier of the operation. | [required] |

### Return type

[**crate::models::OperationGet**](OperationGet.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_get_operation_tag

Get a specific operation tag by its unique identifier.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityGetOperationTagParams {
    // parameters
};
api_security_get_operation_tag(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**tag_id** | **String** | The unique identifier of the operation tag. | [required] |

### Return type

[**crate::models::TagGet**](TagGet.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_list_discovered_operations

List all discovered operations associated with a specific service. Optionally filter operations by status.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityListDiscoveredOperationsParams {
    // parameters
};
api_security_list_discovered_operations(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**method** | Option\<[**Vec&lt;String&gt;**](String.md)> | Filter operations by HTTP method. |  |
**domain** | Option\<[**Vec&lt;String&gt;**](String.md)> | Filter operations by fully-qualified domain name (exact match). |  |
**path** | Option\<**String**> | Filter operations by path (exact match). |  |
**limit** | Option\<**i32**> | The maximum number of operations to return per page. |  |[default to 100]
**page** | Option\<**i32**> | The page number to return. |  |[default to 0]

### Return type

[**crate::models::InlineResponse2001**](InlineResponse2001.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_list_operation_tags

List all operation tags associated with a specific service.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityListOperationTagsParams {
    // parameters
};
api_security_list_operation_tags(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**limit** | Option\<**i32**> | The maximum number of operations to return per page. |  |[default to 100]
**page** | Option\<**i32**> | The page number to return. |  |[default to 0]

### Return type

[**crate::models::InlineResponse2003**](InlineResponse2003.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_list_operations

List all operations associated with a specific service. Optionally filter operations by tag ID.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityListOperationsParams {
    // parameters
};
api_security_list_operations(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**tag_id** | Option\<**String**> | Filter operations by operation tag ID. Only operations associated with this operation tag will be returned. |  |
**status** | Option\<**String**> | Filter operations by status. Defaults to SAVED if omitted. |  |[default to SAVED]
**method** | Option\<[**Vec&lt;String&gt;**](String.md)> | Filter operations by HTTP method. |  |
**domain** | Option\<[**Vec&lt;String&gt;**](String.md)> | Filter operations by fully-qualified domain name (exact match). |  |
**path** | Option\<**String**> | Filter operations by path (exact match). |  |
**limit** | Option\<**i32**> | The maximum number of operations to return per page. |  |[default to 100]
**page** | Option\<**i32**> | The page number to return. |  |[default to 0]

### Return type

[**crate::models::InlineResponse2002**](InlineResponse2002.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_update_operation

Partially update an existing operation associated with a specific service.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityUpdateOperationParams {
    // parameters
};
api_security_update_operation(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**operation_id** | **String** | The unique identifier of the operation. | [required] |
**operation_update** | Option\<[**OperationUpdate**](OperationUpdate.md)> |  |  |

### Return type

[**crate::models::OperationGet**](OperationGet.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## api_security_update_operation_tag

Partially update an existing operation tag.

```rust
let cfg = &Configuration::default();
let params = ApiSecurityUpdateOperationTagParams {
    // parameters
};
api_security_update_operation_tag(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | The unique identifier of the service. | [required] |
**tag_id** | **String** | The unique identifier of the operation tag. | [required] |
**body** | Option\<**crate::models::TagBase**> |  |  |

### Return type

[**crate::models::TagGet**](TagGet.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

