# TlsBulkCertificateResponseAttributesAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**not_after** | Option<**String**> | Time-stamp (GMT) when the certificate will expire. Must be in the future to be used to terminate TLS traffic. | [readonly]
**not_before** | Option<**String**> | Time-stamp (GMT) when the certificate will become valid. Must be in the past to be used to terminate TLS traffic. | [readonly]
**replace** | Option<**bool**> | A recommendation from Fastly indicating the key associated with this certificate is in need of rotation. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


