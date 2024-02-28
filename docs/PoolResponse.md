# PoolResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tls_ca_cert** | Option<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. | [default to null]
**tls_client_cert** | Option<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. | [default to null]
**tls_client_key** | Option<**String**> | The client private key used to make authenticated requests. Must be in PEM format. | [default to null]
**tls_cert_hostname** | Option<**String**> | The hostname used to verify a server's certificate. It can either be the Common Name (CN) or a Subject Alternative Name (SAN). | [default to null]
**use_tls** | Option<**String**> | Whether to use TLS. | [default to UseTls_no_tls]
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**String**> |  | [readonly]
**name** | Option<**String**> | Name for the Pool. | 
**shield** | Option<**String**> | Selected POP to serve as a shield for the servers. Defaults to `null` meaning no origin shielding if not set. Refer to the [POPs API endpoint](https://www.fastly.com/documentation/reference/api/utils/pops/) to get a list of available POPs used for shielding. | [default to null]
**request_condition** | Option<**String**> | Condition which, if met, will select this configuration during a request. Optional. | 
**tls_ciphers** | Option<**String**> | List of OpenSSL ciphers (see the [openssl.org manpages](https://www.openssl.org/docs/man1.1.1/man1/ciphers.html) for details). Optional. | 
**tls_sni_hostname** | Option<**String**> | SNI hostname. Optional. | 
**min_tls_version** | Option<**i32**> | Minimum allowed TLS version on connections to this server. Optional. | 
**max_tls_version** | Option<**i32**> | Maximum allowed TLS version on connections to this server. Optional. | 
**healthcheck** | Option<**String**> | Name of the healthcheck to use with this pool. Can be empty and could be reused across multiple backend and pools. | 
**comment** | Option<**String**> | A freeform descriptive note. | 
**_type** | Option<**String**> | What type of load balance group to use. | 
**override_host** | Option<**String**> | The hostname to [override the Host header](https://docs.fastly.com/en/guides/specifying-an-override-host). Defaults to `null` meaning no override of the Host header will occur. This setting can also be added to a Server definition. If the field is set on a Server definition it will override the Pool setting. | [default to null]
**between_bytes_timeout** | Option<**String**> | Maximum duration in milliseconds that Fastly will wait while receiving no data on a download from a backend. If exceeded, the response received so far will be considered complete and the fetch will end. May be set at runtime using `bereq.between_bytes_timeout`. | 
**connect_timeout** | Option<**String**> | How long to wait for a timeout in milliseconds. | 
**first_byte_timeout** | Option<**String**> | How long to wait for the first byte in milliseconds. | 
**max_conn_default** | Option<**String**> | Maximum number of connections. | [default to 200]
**tls_check_cert** | Option<**String**> | Be strict on checking TLS certs. | 
**id** | Option<**String**> |  | [readonly]
**quorum** | Option<**String**> | Percentage of capacity (`0-100`) that needs to be operationally available for a pool to be considered up. | [default to 75]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


