# UserApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UserApi.md#create_user) | **POST** /user | Create a user
[**delete_user**](UserApi.md#delete_user) | **DELETE** /user/{user_id} | Delete a user
[**get_current_user**](UserApi.md#get_current_user) | **GET** /current_user | Get the current user
[**get_user**](UserApi.md#get_user) | **GET** /user/{user_id} | Get a user
[**request_password_reset**](UserApi.md#request_password_reset) | **POST** /user/{user_login}/password/request_reset | Request a password reset
[**update_user**](UserApi.md#update_user) | **PUT** /user/{user_id} | Update a user
[**update_user_password**](UserApi.md#update_user_password) | **POST** /current_user/password | Update the user's password



## create_user

Create a user.

```rust
let cfg = &Configuration::default();
let params = CreateUserParams {
    // parameters
};
create_user(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login** | Option\<**String**> |  |  |
**name** | Option\<**String**> | The real life name of the user. |  |
**limit_services** | Option\<**bool**> | Indicates that the user has limited access to the customer's services. |  |
**locked** | Option\<**bool**> | Indicates whether the is account is locked for editing or not. |  |
**require_new_password** | Option\<**bool**> | Indicates if a new password is required at next login. |  |
**role** | Option\<[**crate::models::RoleUser**](role_user.md)> |  |  |
**two_factor_auth_enabled** | Option\<**bool**> | Indicates if 2FA is enabled on the user. |  |
**two_factor_setup_required** | Option\<**bool**> | Indicates if 2FA is required by the user's customer account. |  |

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_user

Delete a user.

```rust
let cfg = &Configuration::default();
let params = DeleteUserParams {
    // parameters
};
delete_user(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Alphanumeric string identifying the user. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_current_user

Get the logged in user.

```rust
let cfg = &Configuration::default();
let params = GetCurrentUserParams {
    // parameters
};
get_current_user(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_user

Get a specific user.

```rust
let cfg = &Configuration::default();
let params = GetUserParams {
    // parameters
};
get_user(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Alphanumeric string identifying the user. | [required] |

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## request_password_reset

Requests a password reset for the specified user.

```rust
let cfg = &Configuration::default();
let params = RequestPasswordResetParams {
    // parameters
};
request_password_reset(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_login** | **String** | The login associated with the user (typically, an email address). | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_user

Update a user. Only users with the role of `superuser` can make changes to other users on the account. Non-superusers may use this endpoint to make changes to their own account. Two-factor attributes are not editable via this endpoint.

```rust
let cfg = &Configuration::default();
let params = UpdateUserParams {
    // parameters
};
update_user(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Alphanumeric string identifying the user. | [required] |
**login** | Option\<**String**> |  |  |
**name** | Option\<**String**> | The real life name of the user. |  |
**limit_services** | Option\<**bool**> | Indicates that the user has limited access to the customer's services. |  |
**locked** | Option\<**bool**> | Indicates whether the is account is locked for editing or not. |  |
**require_new_password** | Option\<**bool**> | Indicates if a new password is required at next login. |  |
**role** | Option\<[**crate::models::RoleUser**](role_user.md)> |  |  |
**two_factor_auth_enabled** | Option\<**bool**> | Indicates if 2FA is enabled on the user. |  |
**two_factor_setup_required** | Option\<**bool**> | Indicates if 2FA is required by the user's customer account. |  |

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_user_password

Update the user's password to a new one.

```rust
let cfg = &Configuration::default();
let params = UpdateUserPasswordParams {
    // parameters
};
update_user_password(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**old_password** | Option\<**String**> | The user's current password. |  |
**new_password** | Option\<**String**> | The user's new password. |  |

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

[session_password_change](../README.md#session_password_change)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

