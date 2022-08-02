# TlsCertificateResponseAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**issued_to** | Option<**String**> | The hostname for which a certificate was issued. | [readonly]
**issuer** | Option<**String**> | The certificate authority that issued the certificate. | [readonly]
**serial_number** | Option<**String**> | A value assigned by the issuer that is unique to a certificate. | [readonly]
**signature_algorithm** | Option<**String**> | The algorithm used to sign the certificate. | [readonly]
**not_after** | Option<**String**> | Time-stamp (GMT) when the certificate will expire. Must be in the future to be used to terminate TLS traffic. | [readonly]
**not_before** | Option<**String**> | Time-stamp (GMT) when the certificate will become valid. Must be in the past to be used to terminate TLS traffic. | [readonly]
**replace** | Option<**bool**> | A recommendation from Fastly indicating the key associated with this certificate is in need of rotation. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


