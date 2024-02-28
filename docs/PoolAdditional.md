# PoolAdditional

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
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

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


