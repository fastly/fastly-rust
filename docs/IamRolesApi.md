# IamRolesApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**iam_v1_roles_get**](IamRolesApi.md#iam_v1_roles_get) | **GET** /iam/v1/roles/{role_id} | Get IAM role by ID
[**iam_v1_roles_list**](IamRolesApi.md#iam_v1_roles_list) | **GET** /iam/v1/roles | List IAM roles



## iam_v1_roles_get

Retrieve a single IAM role by its unique identifier. 

```rust
let cfg = &Configuration::default();
let params = IamV1RolesGetParams {
    // parameters
};
iam_v1_roles_get(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Alphanumeric string identifying the role. | [required] |
**include** | Option\<**String**> | Include related data (i.e., permissions). |  |

### Return type

[**crate::models::IamV1RoleResponse**](IamV1RoleResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## iam_v1_roles_list

Retrieve a paginated list of IAM roles available in the account. 

```rust
let cfg = &Configuration::default();
let params = IamV1RolesListParams {
    // parameters
};
iam_v1_roles_list(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option\<**i32**> | Number of results per page. The maximum is 1000. |  |[default to 100]
**cursor** | Option\<**String**> | Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty. |  |

### Return type

[**crate::models::IamV1RoleListResponse**](IamV1RoleListResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

