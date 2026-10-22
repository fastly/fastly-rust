# AiRuntimeControlVirtualKeysApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**arc_create_virtual_key**](AiRuntimeControlVirtualKeysApi.md#arc_create_virtual_key) | **POST** /ai-runtime-control/v1/keys | Create a virtual key
[**arc_delete_virtual_key**](AiRuntimeControlVirtualKeysApi.md#arc_delete_virtual_key) | **DELETE** /ai-runtime-control/v1/keys/{key_id} | Delete a virtual key
[**arc_get_virtual_key**](AiRuntimeControlVirtualKeysApi.md#arc_get_virtual_key) | **GET** /ai-runtime-control/v1/keys/{key_id} | Get a virtual key
[**arc_get_virtual_key_failover**](AiRuntimeControlVirtualKeysApi.md#arc_get_virtual_key_failover) | **GET** /ai-runtime-control/v1/keys/{key_id}/failover | Get a virtual key's failover policy
[**arc_list_virtual_keys**](AiRuntimeControlVirtualKeysApi.md#arc_list_virtual_keys) | **GET** /ai-runtime-control/v1/keys | List virtual keys
[**arc_rotate_virtual_key**](AiRuntimeControlVirtualKeysApi.md#arc_rotate_virtual_key) | **POST** /ai-runtime-control/v1/keys/{key_id}/rotate | Rotate a virtual key
[**arc_update_virtual_key**](AiRuntimeControlVirtualKeysApi.md#arc_update_virtual_key) | **PATCH** /ai-runtime-control/v1/keys/{key_id} | Update a virtual key
[**arc_update_virtual_key_failover**](AiRuntimeControlVirtualKeysApi.md#arc_update_virtual_key_failover) | **PUT** /ai-runtime-control/v1/keys/{key_id}/failover | Update a virtual key's failover policy



## arc_create_virtual_key

Create a new virtual key for authenticating requests through the AI Runtime Control gateway.

```rust
let cfg = &Configuration::default();
let params = ArcCreateVirtualKeyParams {
    // parameters
};
arc_create_virtual_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**arc_virtual_key** | [**ArcVirtualKey**](ArcVirtualKey.md) |  | [required] |

### Return type

[**crate::models::ArcCreateVirtualKeyResponse**](ArcCreateVirtualKeyResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_delete_virtual_key

Delete a virtual key. This also deletes any security configuration associated with the key.

```rust
let cfg = &Configuration::default();
let params = ArcDeleteVirtualKeyParams {
    // parameters
};
arc_delete_virtual_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The ID of the virtual key. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_get_virtual_key

Retrieve a single virtual key.

```rust
let cfg = &Configuration::default();
let params = ArcGetVirtualKeyParams {
    // parameters
};
arc_get_virtual_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The ID of the virtual key. | [required] |

### Return type

[**crate::models::ArcVirtualKeyResponse**](ArcVirtualKeyResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_get_virtual_key_failover

Retrieve the failover policy for a virtual key. A failover policy lists alternate provider connections to retry a request against, and which error conditions trigger a retry.  Failover is a per-account feature. If it isn't enabled on your account, this endpoint returns a 404 response. 

```rust
let cfg = &Configuration::default();
let params = ArcGetVirtualKeyFailoverParams {
    // parameters
};
arc_get_virtual_key_failover(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The ID of the virtual key. | [required] |

### Return type

[**crate::models::ArcVirtualKeyFailoverResponse**](ArcVirtualKeyFailoverResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_list_virtual_keys

List the virtual keys belonging to your account.

```rust
let cfg = &Configuration::default();
let params = ArcListVirtualKeysParams {
    // parameters
};
arc_list_virtual_keys(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deleted** | Option\<**bool**> | Include deleted virtual keys in the results. |  |[default to false]
**search** | Option\<**String**> | Filter results to virtual keys whose name contains this substring. |  |
**cursor** | Option\<**String**> | Cursor for pagination. Use the `next_cursor` value from a previous response to request the next page of results, or the `prev_cursor` value to request the previous page.  |  |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 10]
**sort** | Option\<**String**> | Sort field. Prefix with `-` for descending order (e.g., `sort=-created_at`).  |  |[default to created_at]

### Return type

[**crate::models::ArcVirtualKeysResponse**](ArcVirtualKeysResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_rotate_virtual_key

Issue a new access token for a virtual key, invalidating the previous one.

```rust
let cfg = &Configuration::default();
let params = ArcRotateVirtualKeyParams {
    // parameters
};
arc_rotate_virtual_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The ID of the virtual key. | [required] |
**arc_rotate_virtual_key_request** | Option\<[**ArcRotateVirtualKeyRequest**](ArcRotateVirtualKeyRequest.md)> |  |  |

### Return type

[**crate::models::ArcRotateVirtualKeyResponse**](ArcRotateVirtualKeyResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_update_virtual_key

Update the attributes of a virtual key. Only the fields you include are changed.

```rust
let cfg = &Configuration::default();
let params = ArcUpdateVirtualKeyParams {
    // parameters
};
arc_update_virtual_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The ID of the virtual key. | [required] |
**arc_update_virtual_key_request** | [**ArcUpdateVirtualKeyRequest**](ArcUpdateVirtualKeyRequest.md) |  | [required] |

### Return type

[**crate::models::ArcUpdateVirtualKeyResponse**](ArcUpdateVirtualKeyResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_update_virtual_key_failover

Replace the failover policy for a virtual key.  Failover is a per-account feature. If it isn't enabled on your account, this endpoint returns a 404 response. 

```rust
let cfg = &Configuration::default();
let params = ArcUpdateVirtualKeyFailoverParams {
    // parameters
};
arc_update_virtual_key_failover(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The ID of the virtual key. | [required] |
**arc_virtual_key_failover_request** | [**ArcVirtualKeyFailoverRequest**](ArcVirtualKeyFailoverRequest.md) |  | [required] |

### Return type

[**crate::models::ArcVirtualKeyFailoverResponse**](ArcVirtualKeyFailoverResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

