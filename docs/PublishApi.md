# PublishApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**publish**](PublishApi.md#publish) | **POST** /service/{service_id}/publish/ | Send messages to Fanout subscribers



## publish

Send one or more messages to [Fanout](https://developer.fastly.com/learning/concepts/real-time-messaging/fanout) subscribers. Each message specifies a channel, and Fanout will deliver the message to all subscribers of its channel. > **IMPORTANT:** For compatibility with GRIP, this endpoint requires a trailing slash, and the API token may be provided in the `Authorization` header (instead of the `Fastly-Key` header) using the `Bearer` scheme. 

```rust
let cfg = &Configuration::default();
let params = PublishParams {
    // parameters
};
publish(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**publish_request** | Option\<[**PublishRequest**](PublishRequest.md)> |  |  |

### Return type

**String**

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

