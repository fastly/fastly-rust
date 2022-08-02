# TlsPrivateKeyResponseAttributesAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | A customizable name for your private key. | 
**key_length** | Option<**i32**> | The key length used to generate the private key. | [readonly]
**key_type** | Option<**String**> | The algorithm used to generate the private key. Must be `RSA`. | [readonly]
**replace** | Option<**bool**> | A recommendation from Fastly to replace this private key and all associated certificates. | [readonly]
**public_key_sha1** | Option<**String**> | Useful for safely identifying the key. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


