# ArcProviderConnectionApiKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | A display name for the provider connection. Cannot match a name reserved by a built-in provider. | 
**models** | **Vec<String>** | The AI model identifiers this connection is allowed to use. | 
**base_url** | **String** | The base URL of the provider's API. Must be an absolute `http` or `https` URL and must not end in `/v1`. | 
**api_key** | **String** | The provider's secret API key. | 
**auth_type** | Option<**String**> | The authentication type for this connection. Omit for the default `api-key` behavior. | [default to AuthType_ApiKey]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


