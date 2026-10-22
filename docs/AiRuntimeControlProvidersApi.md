# AiRuntimeControlProvidersApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**arc_get_bedrock_setup**](AiRuntimeControlProvidersApi.md#arc_get_bedrock_setup) | **GET** /ai-runtime-control/v1/providers/bedrock/setup | Get Bedrock setup information
[**arc_get_models_for_provider**](AiRuntimeControlProvidersApi.md#arc_get_models_for_provider) | **GET** /ai-runtime-control/v1/providers/{provider_id}/models | Get models for a provider
[**arc_get_supported_providers**](AiRuntimeControlProvidersApi.md#arc_get_supported_providers) | **GET** /ai-runtime-control/v1/providers | Get supported providers



## arc_get_bedrock_setup

Returns the AI Runtime Control IAM role Amazon Resource Name (ARN) and external ID to reference in your AWS account's IAM trust policy before creating an `aws-iam` [provider connection](https://www.fastly.com/documentation/reference/api/ai-runtime-control/provider-connections#arc-create-provider-connection) to Amazon Bedrock.  Requires AWS IAM authentication to be enabled on your account. If it isn't, this endpoint returns a 404 response. 

```rust
let cfg = &Configuration::default();
let params = ArcGetBedrockSetupParams {
    // parameters
};
arc_get_bedrock_setup(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ArcBedrockSetupResponse**](ArcBedrockSetupResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_get_models_for_provider

Returns the models available for a specific provider.

```rust
let cfg = &Configuration::default();
let params = ArcGetModelsForProviderParams {
    // parameters
};
arc_get_models_for_provider(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_id** | **String** | The ID of the provider. | [required] |

### Return type

[**crate::models::ArcModelsResponse**](ArcModelsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_get_supported_providers

Returns the AI providers supported by AI Runtime Control, with each provider's available models nested within.

```rust
let cfg = &Configuration::default();
let params = ArcGetSupportedProvidersParams {
    // parameters
};
arc_get_supported_providers(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ArcProvidersResponse**](ArcProvidersResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

