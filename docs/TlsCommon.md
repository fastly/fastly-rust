# TlsCommon

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tls_ca_cert** | Option<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. | [default to null]
**tls_client_cert** | Option<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. | [default to null]
**tls_client_key** | Option<**String**> | The client private key used to make authenticated requests. Must be in PEM format. | [default to null]
**tls_cert_hostname** | Option<**String**> | The hostname used to verify a server's certificate. It can either be the Common Name (CN) or a Subject Alternative Name (SAN). | [default to null]
**use_tls** | Option<**i32**> | Whether to use TLS. | [default to UseTls_no_tls]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


