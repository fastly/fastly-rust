# EventsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_event**](EventsApi.md#get_event) | **GET** /events/{event_id} | Get an event
[**list_events**](EventsApi.md#list_events) | **GET** /events | List events



## get_event

Get a specific event.

```rust
let cfg = &Configuration::default();
let params = GetEventParams {
    // parameters
};
get_event(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | Alphanumeric string identifying an event. | [required] |

### Return type

[**crate::models::EventResponse**](EventResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_events

List all events for a particular customer. Events can be filtered by user, customer and event type. Events can be sorted by date.

```rust
let cfg = &Configuration::default();
let params = ListEventsParams {
    // parameters
};
list_events(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_customer_id** | Option\<**String**> | Limit the results returned to a specific customer. |  |
**filter_event_type** | Option\<**String**> | Limit the returned events to a specific `event_type`. |  |
**filter_service_id** | Option\<**String**> | Limit the results returned to a specific service. |  |
**filter_user_id** | Option\<**String**> | Limit the results returned to a specific user. |  |
**filter_token_id** | Option\<**String**> | Limit the returned events to a specific token. |  |
**filter_created_at** | Option\<**String**> | Limit the returned events to a specific time frame. Accepts sub-parameters: lt, lte, gt, gte (e.g., filter[created_at][gt]=2022-01-12).  |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**sort** | Option\<**String**> | The order in which to list the results by creation date. |  |[default to created_at]

### Return type

[**crate::models::EventsResponse**](EventsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

