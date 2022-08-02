# TlsConfigurationResponseAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**default** | Option<**bool**> | Signifies whether or not Fastly will use this configuration as a default when creating a new [TLS Activation](/reference/api/tls/custom-certs/activations/). | [readonly]
**http_protocols** | Option<**Vec<String>**> | HTTP protocols available on your configuration. | [readonly]
**tls_protocols** | Option<**Vec<f32>**> | TLS protocols available on your configuration. | [readonly]
**bulk** | Option<**bool**> | Signifies whether the configuration is used for Platform TLS or not. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


