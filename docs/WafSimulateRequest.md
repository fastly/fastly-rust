# WafSimulateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request** | **String** | The raw HTTP request in wire format to simulate through the WAF. Must include the request line, headers, and optionally a body, separated by CRLF sequences. | 
**response** | Option<**String**> | The raw HTTP response in wire format. The WAF engine inspects response headers during its PostRequest phase and may generate signals from them. When omitted, a default response of `HTTP/1.1 200 OK\\r\\n\\r\\n` is used. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


