# ArcUpdateProviderConnectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**models** | Option<**Vec<String>**> | The AI model identifiers this connection is allowed to use. Must be non-empty if included. | 
**base_url** | Option<**String**> | The base URL of the provider's API (`api-key` connections only). | 
**api_key** | Option<**String**> | The provider's secret API key (`api-key` connections only). | 
**customer_role_arn** | Option<**String**> | The ARN of the IAM role to assume for Bedrock requests (`aws-iam` connections only). | 
**region** | Option<**String**> | The AWS region for Bedrock API requests (`aws-iam` connections only). | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


