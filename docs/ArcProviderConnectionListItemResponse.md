# ArcProviderConnectionListItemResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the provider connection. | [readonly]
**secret_id** | **String** | An opaque identifier for the secret material backing this connection. Not the secret itself. | [readonly]
**name** | **String** | The display name of the provider connection. | 
**models** | **Vec<String>** | The AI model identifiers this connection is allowed to use. | 
**base_url** | Option<**String**> | The base URL of the provider's API. Present only for `api-key` connections. | 
**auth_type** | **String** | The authentication type for this connection. | 
**region** | Option<**String**> | The AWS region for Bedrock API requests. Present only for `aws-iam` connections. | 
**created_at** | **String** | The date and time the connection was created, in RFC 3339 format. | [readonly]
**updated_at** | **String** | The date and time the connection was last updated, in RFC 3339 format. | [readonly]
**deleted_at** | Option<**String**> | The date and time the connection was deleted, in RFC 3339 format. Present only for deleted connections, which are returned only when `include_deleted` is `true`. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


