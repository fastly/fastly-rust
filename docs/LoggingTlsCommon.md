# LoggingTlsCommon

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tls_ca_cert** | Option<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. | [default to null]
**tls_client_cert** | Option<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. | [default to null]
**tls_client_key** | Option<**String**> | The client private key used to make authenticated requests. Must be in PEM format. | [default to null]
**tls_hostname** | Option<**String**> | The hostname to verify the server's certificate. This should be one of the Subject Alternative Name (SAN) fields for the certificate. Common Names (CN) are not supported. | [default to null]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


