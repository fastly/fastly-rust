# Backend

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | A hostname, IPv4, or IPv6 address for the backend. This is the preferred way to specify the location of your backend. | 
**auto_loadbalance** | Option<**bool**> | Whether or not this backend should be automatically load balanced. If true, all backends with this setting that don't have a `request_condition` will be selected based on their `weight`. | 
**between_bytes_timeout** | Option<**i32**> | Maximum duration in milliseconds that Fastly will wait while receiving no data on a download from a backend. If exceeded, the response received so far will be considered complete and the fetch will end. May be set at runtime using `bereq.between_bytes_timeout`. | 
**client_cert** | Option<**String**> | Unused. | 
**comment** | Option<**String**> | A freeform descriptive note. | 
**connect_timeout** | Option<**i32**> | Maximum duration in milliseconds to wait for a connection to this backend to be established. If exceeded, the connection is aborted and a synthetic `503` response will be presented instead. May be set at runtime using `bereq.connect_timeout`. | 
**first_byte_timeout** | Option<**i32**> | Maximum duration in milliseconds to wait for the server response to begin after a TCP connection is established and the request has been sent. If exceeded, the connection is aborted and a synthetic `503` response will be presented instead. May be set at runtime using `bereq.first_byte_timeout`. | 
**healthcheck** | Option<**String**> | The name of the healthcheck to use with this backend. | 
**hostname** | Option<**String**> | The hostname of the backend. May be used as an alternative to `address` to set the backend location. | 
**ipv4** | Option<**String**> | IPv4 address of the backend. May be used as an alternative to `address` to set the backend location. | 
**ipv6** | Option<**String**> | IPv6 address of the backend. May be used as an alternative to `address` to set the backend location. | 
**keepalive_time** | Option<**i32**> | How long in seconds to keep a persistent connection to the backend between requests. By default, Varnish keeps connections open as long as it can. | 
**max_conn** | Option<**i32**> | Maximum number of concurrent connections this backend will accept. | 
**max_tls_version** | Option<**String**> | Maximum allowed TLS version on SSL connections to this backend. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated. | 
**min_tls_version** | Option<**String**> | Minimum allowed TLS version on SSL connections to this backend. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated. | 
**name** | Option<**String**> | The name of the backend. | 
**override_host** | Option<**String**> | If set, will replace the client-supplied HTTP `Host` header on connections to this backend. Applied after VCL has been processed, so this setting will take precedence over changing `bereq.http.Host` in VCL. | 
**port** | Option<**i32**> | Port on which the backend server is listening for connections from Fastly. Setting `port` to 80 or 443 will also set `use_ssl` automatically (to false and true respectively), unless explicitly overridden by setting `use_ssl` in the same request. | 
**prefer_ipv6** | Option<**bool**> | Prefer IPv6 connections to origins for hostname backends. | 
**request_condition** | Option<**String**> | Name of a Condition, which if satisfied, will select this backend during a request. If set, will override any `auto_loadbalance` setting. By default, the first backend added to a service is selected for all requests. | 
**share_key** | Option<**String**> | Value that when shared across backends will enable those backends to share the same health check. | 
**shield** | Option<**String**> | Identifier of the POP to use as a [shield](https://docs.fastly.com/en/guides/shielding). | 
**ssl_ca_cert** | Option<**String**> | CA certificate attached to origin. | 
**ssl_cert_hostname** | Option<**String**> | Overrides `ssl_hostname`, but only for cert verification. Does not affect SNI at all. | 
**ssl_check_cert** | Option<**bool**> | Be strict on checking SSL certs. | [default to true]
**ssl_ciphers** | Option<**String**> | List of [OpenSSL ciphers](https://www.openssl.org/docs/man1.1.1/man1/ciphers.html) to support for connections to this origin. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated. | 
**ssl_client_cert** | Option<**String**> | Client certificate attached to origin. | 
**ssl_client_key** | Option<**String**> | Client key attached to origin. | 
**ssl_hostname** | Option<**String**> | Use `ssl_cert_hostname` and `ssl_sni_hostname` to configure certificate validation. | 
**ssl_sni_hostname** | Option<**String**> | Overrides `ssl_hostname`, but only for SNI in the handshake. Does not affect cert validation at all. | 
**tcp_keepalive_enable** | Option<**bool**> | Whether to enable TCP keepalives for backend connections. Varnish defaults to using keepalives if this is unspecified. | 
**tcp_keepalive_interval** | Option<**i32**> | Interval in seconds between subsequent keepalive probes. | 
**tcp_keepalive_probes** | Option<**i32**> | Number of unacknowledged probes to send before considering the connection dead. | 
**tcp_keepalive_time** | Option<**i32**> | Interval in seconds between the last data packet sent and the first keepalive probe. | 
**use_ssl** | Option<**bool**> | Whether or not to require TLS for connections to this backend. | 
**weight** | Option<**i32**> | Weight used to load balance this backend against others. May be any positive integer. If `auto_loadbalance` is true, the chance of this backend being selected is equal to its own weight over the sum of all weights for backends that have `auto_loadbalance` set to true. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


