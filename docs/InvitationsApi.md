# InvitationsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_invitation**](InvitationsApi.md#create_invitation) | **POST** /invitations | Create an invitation
[**delete_invitation**](InvitationsApi.md#delete_invitation) | **DELETE** /invitations/{invitation_id} | Delete an invitation
[**list_invitations**](InvitationsApi.md#list_invitations) | **GET** /invitations | List invitations



## create_invitation

Create an invitation.

```rust
let cfg = &Configuration::default();
let params = CreateInvitationParams {
    // parameters
};
create_invitation(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation** | Option\<[**Invitation**](Invitation.md)> |  |  |

### Return type

[**crate::models::InvitationResponse**](InvitationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_invitation

Delete an invitation.

```rust
let cfg = &Configuration::default();
let params = DeleteInvitationParams {
    // parameters
};
delete_invitation(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_id** | **String** | Alphanumeric string identifying an invitation. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_invitations

List all invitations.

```rust
let cfg = &Configuration::default();
let params = ListInvitationsParams {
    // parameters
};
list_invitations(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]

### Return type

[**crate::models::InvitationsResponse**](InvitationsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

