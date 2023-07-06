# ClientKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_key** | Option<**String**> | A Base64-encoded X25519 public key that can be used with a [libsodium-compatible sealed box](https://libsodium.gitbook.io/doc/public-key_cryptography/sealed_boxes) to encrypt secrets before upload. | 
**signature** | Option<**String**> | A Base64-encoded signature of the client key. The signature is generated using the signing key and must be verified before using the client key. | 
**expires_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


