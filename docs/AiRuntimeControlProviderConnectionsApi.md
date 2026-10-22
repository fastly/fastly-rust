# AiRuntimeControlProviderConnectionsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**arc_create_provider_connection**](AiRuntimeControlProviderConnectionsApi.md#arc_create_provider_connection) | **POST** /ai-runtime-control/v1/provider-connections | Create a provider connection
[**arc_delete_provider_connection**](AiRuntimeControlProviderConnectionsApi.md#arc_delete_provider_connection) | **DELETE** /ai-runtime-control/v1/provider-connections/{id} | Delete a provider connection
[**arc_get_provider_connection**](AiRuntimeControlProviderConnectionsApi.md#arc_get_provider_connection) | **GET** /ai-runtime-control/v1/provider-connections/{id} | Get a provider connection
[**arc_list_provider_connections**](AiRuntimeControlProviderConnectionsApi.md#arc_list_provider_connections) | **GET** /ai-runtime-control/v1/provider-connections | List provider connections
[**arc_update_provider_connection**](AiRuntimeControlProviderConnectionsApi.md#arc_update_provider_connection) | **PATCH** /ai-runtime-control/v1/provider-connections/{id} | Update a provider connection



## arc_create_provider_connection

Create a provider connection. Set `auth_type` to `api-key` (the default) to connect with a provider's own API key, or to `aws-iam` to connect to Amazon Bedrock using an AWS IAM role.  Accounts are limited to 25 provider connections. Creating a 26th connection returns a 409 response. 

```rust
let cfg = &Configuration::default();
let params = ArcCreateProviderConnectionParams {
    // parameters
};
arc_create_provider_connection(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**arc_provider_connection_request** | [**ArcProviderConnectionRequest**](ArcProviderConnectionRequest.md) |  | [required] |

### Return type

[**crate::models::ArcProviderConnectionResponse**](ArcProviderConnectionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_delete_provider_connection

Delete a provider connection. Returns a 409 response if the connection is still referenced by a virtual key's failover policy. 

```rust
let cfg = &Configuration::default();
let params = ArcDeleteProviderConnectionParams {
    // parameters
};
arc_delete_provider_connection(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the provider connection. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_get_provider_connection

Retrieve a single provider connection.

```rust
let cfg = &Configuration::default();
let params = ArcGetProviderConnectionParams {
    // parameters
};
arc_get_provider_connection(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the provider connection. | [required] |

### Return type

[**crate::models::ArcGetProviderConnectionResponse**](ArcGetProviderConnectionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_list_provider_connections

List the provider connections belonging to your account.

```rust
let cfg = &Configuration::default();
let params = ArcListProviderConnectionsParams {
    // parameters
};
arc_list_provider_connections(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deleted** | Option\<**bool**> | Include deleted provider connections in the results. Deleted connections have a `deleted_at` value. |  |[default to false]
**sort** | Option\<**String**> | Sort field. Prefix with `-` for descending order (e.g., `sort=-name`).  |  |[default to created_at]

### Return type

[**crate::models::ArcProviderConnectionsResponse**](ArcProviderConnectionsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_update_provider_connection

Update the attributes of a provider connection. Only the fields you include are changed. Which fields you can set depends on the connection's existing `auth_type`: `base_url` and `api_key` apply only to `api-key` connections, and `customer_role_arn` and `region` apply only to `aws-iam` connections. `auth_type` itself cannot be changed. 

```rust
let cfg = &Configuration::default();
let params = ArcUpdateProviderConnectionParams {
    // parameters
};
arc_update_provider_connection(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the provider connection. | [required] |
**arc_update_provider_connection_request** | [**ArcUpdateProviderConnectionRequest**](ArcUpdateProviderConnectionRequest.md) |  | [required] |

### Return type

[**crate::models::ArcProviderConnectionResponse**](ArcProviderConnectionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

