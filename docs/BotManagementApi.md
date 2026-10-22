# BotManagementApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**bot_management_get_bot**](BotManagementApi.md#bot_management_get_bot) | **GET** /bot-management/v1/workspaces/{workspace_id}/policy/categories/{category_id}/bots/{bot_id} | Get a bot
[**bot_management_get_category**](BotManagementApi.md#bot_management_get_category) | **GET** /bot-management/v1/workspaces/{workspace_id}/policy/categories/{category_id} | Get a category
[**bot_management_get_workspace**](BotManagementApi.md#bot_management_get_workspace) | **GET** /bot-management/v1/workspaces/{workspace_id} | Get a workspace
[**bot_management_list_bots**](BotManagementApi.md#bot_management_list_bots) | **GET** /bot-management/v1/workspaces/{workspace_id}/policy/bots | List all bots in a workspace
[**bot_management_list_categories**](BotManagementApi.md#bot_management_list_categories) | **GET** /bot-management/v1/workspaces/{workspace_id}/policy/categories | List categories
[**bot_management_list_category_bots**](BotManagementApi.md#bot_management_list_category_bots) | **GET** /bot-management/v1/workspaces/{workspace_id}/policy/categories/{category_id}/bots | List bots in a category
[**bot_management_list_workspaces**](BotManagementApi.md#bot_management_list_workspaces) | **GET** /bot-management/v1/workspaces | List workspaces
[**bot_management_update_bot**](BotManagementApi.md#bot_management_update_bot) | **PATCH** /bot-management/v1/workspaces/{workspace_id}/policy/categories/{category_id}/bots/{bot_id} | Set a bot action
[**bot_management_update_category**](BotManagementApi.md#bot_management_update_category) | **PATCH** /bot-management/v1/workspaces/{workspace_id}/policy/categories/{category_id} | Set a category action
[**bot_management_update_workspace**](BotManagementApi.md#bot_management_update_workspace) | **PATCH** /bot-management/v1/workspaces/{workspace_id} | Update a workspace



## bot_management_get_bot

Get the attributes of a bot in a category.

```rust
let cfg = &Configuration::default();
let params = BotManagementGetBotParams {
    // parameters
};
bot_management_get_bot(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The ID of the bot management workspace. | [required] |
**category_id** | **String** | The ID of the bot category. | [required] |
**bot_id** | **String** | The ID of the bot. | [required] |

### Return type

[**crate::models::BotManagementBotResponse**](BotManagementBotResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## bot_management_get_category

Get the attributes of a bot category.

```rust
let cfg = &Configuration::default();
let params = BotManagementGetCategoryParams {
    // parameters
};
bot_management_get_category(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The ID of the bot management workspace. | [required] |
**category_id** | **String** | The ID of the bot category. | [required] |

### Return type

[**crate::models::BotManagementCategoryResponse**](BotManagementCategoryResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## bot_management_get_workspace

Retrieve a bot management workspace.

```rust
let cfg = &Configuration::default();
let params = BotManagementGetWorkspaceParams {
    // parameters
};
bot_management_get_workspace(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The ID of the bot management workspace. | [required] |

### Return type

[**crate::models::BotManagementWorkspaceResponse**](BotManagementWorkspaceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## bot_management_list_bots

List all bots in a workspace's policy across all categories.

```rust
let cfg = &Configuration::default();
let params = BotManagementListBotsParams {
    // parameters
};
bot_management_list_bots(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The ID of the bot management workspace. | [required] |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 100]
**cursor** | Option\<**String**> | Cursor for pagination. Use the `next_cursor` value from a previous response to request the next page of results. |  |

### Return type

[**crate::models::BotManagementWorkspaceBotsResponse**](BotManagementWorkspaceBotsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## bot_management_list_categories

List all bot categories in a workspace's policy. Each category has an action that applies to every bot in the category. To list the bots themselves, use [list all bots in a workspace](#bot-management-list-bots) or [list bots in a category](#bot-management-list-category-bots).

```rust
let cfg = &Configuration::default();
let params = BotManagementListCategoriesParams {
    // parameters
};
bot_management_list_categories(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The ID of the bot management workspace. | [required] |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 100]
**cursor** | Option\<**String**> | Cursor for pagination. Use the `next_cursor` value from a previous response to request the next page of results. |  |

### Return type

[**crate::models::BotManagementCategoriesResponse**](BotManagementCategoriesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## bot_management_list_category_bots

List all bots in a bot category.

```rust
let cfg = &Configuration::default();
let params = BotManagementListCategoryBotsParams {
    // parameters
};
bot_management_list_category_bots(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The ID of the bot management workspace. | [required] |
**category_id** | **String** | The ID of the bot category. | [required] |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 100]
**cursor** | Option\<**String**> | Cursor for pagination. Use the `next_cursor` value from a previous response to request the next page of results. |  |

### Return type

[**crate::models::BotManagementBotsResponse**](BotManagementBotsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## bot_management_list_workspaces

List bot management workspaces.

```rust
let cfg = &Configuration::default();
let params = BotManagementListWorkspacesParams {
    // parameters
};
bot_management_list_workspaces(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | Option\<**String**> | Filter results to the workspace attached to this service. Returns an empty list if the service is not attached to a workspace. |  |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 100]
**cursor** | Option\<**String**> | Cursor for pagination. Use the `next_cursor` value from a previous response to request the next page of results. |  |

### Return type

[**crate::models::BotManagementWorkspacesResponse**](BotManagementWorkspacesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## bot_management_update_bot

Set the action for a bot. Set the action to 'inherit' to apply the category's action or to a specific value to override the category's action.

```rust
let cfg = &Configuration::default();
let params = BotManagementUpdateBotParams {
    // parameters
};
bot_management_update_bot(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The ID of the bot management workspace. | [required] |
**category_id** | **String** | The ID of the bot category. | [required] |
**bot_id** | **String** | The ID of the bot. | [required] |
**bot_management_bot** | [**BotManagementBot**](BotManagementBot.md) |  | [required] |

### Return type

[**crate::models::BotManagementBotResponse**](BotManagementBotResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## bot_management_update_category

Set the action for a bot category. Every bot in the category configured with the 'inherit' action uses the action set for the category.

```rust
let cfg = &Configuration::default();
let params = BotManagementUpdateCategoryParams {
    // parameters
};
bot_management_update_category(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The ID of the bot management workspace. | [required] |
**category_id** | **String** | The ID of the bot category. | [required] |
**bot_management_category** | [**BotManagementCategory**](BotManagementCategory.md) |  | [required] |

### Return type

[**crate::models::BotManagementCategoryResponse**](BotManagementCategoryResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## bot_management_update_workspace

Update the attributes of a bot management workspace.

```rust
let cfg = &Configuration::default();
let params = BotManagementUpdateWorkspaceParams {
    // parameters
};
bot_management_update_workspace(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The ID of the bot management workspace. | [required] |
**bot_management_workspace** | [**BotManagementWorkspace**](BotManagementWorkspace.md) |  | [required] |

### Return type

[**crate::models::BotManagementWorkspaceResponse**](BotManagementWorkspaceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

