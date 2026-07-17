# NgwafAgentKeysApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**ngwaf_list_agent_keys**](NgwafAgentKeysApi.md#ngwaf_list_agent_keys) | **GET** /ngwaf/v1/workspaces/{workspace_id}/agent-keys | List agent keys for a workspace



## ngwaf_list_agent_keys

List agent keys for a workspace.

```rust
let cfg = &Configuration::default();
let params = NgwafListAgentKeysParams {
    // parameters
};
ngwaf_list_agent_keys(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The ID of the workspace. | [required] |

### Return type

[**crate::models::InlineResponse20019**](InlineResponse20019.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

