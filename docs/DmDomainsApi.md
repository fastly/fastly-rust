# DmDomainsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**create_dm_domain**](DmDomainsApi.md#create_dm_domain) | **POST** /domain-management/v1/domains | Create a domain
[**delete_dm_domain**](DmDomainsApi.md#delete_dm_domain) | **DELETE** /domain-management/v1/domains/{domain_id} | Delete a domain
[**get_dm_domain**](DmDomainsApi.md#get_dm_domain) | **GET** /domain-management/v1/domains/{domain_id} | Get a domain
[**list_dm_domains**](DmDomainsApi.md#list_dm_domains) | **GET** /domain-management/v1/domains | List domains
[**update_dm_domain**](DmDomainsApi.md#update_dm_domain) | **PATCH** /domain-management/v1/domains/{domain_id} | Update a domain



## create_dm_domain

Create a domain

```rust
let cfg = &Configuration::default();
let params = CreateDmDomainParams {
    // parameters
};
create_dm_domain(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body_for_create** | Option\<[**RequestBodyForCreate**](RequestBodyForCreate.md)> |  |  |

### Return type

[**crate::models::SuccessfulResponseAsObject**](SuccessfulResponseAsObject.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_dm_domain

Delete a domain

```rust
let cfg = &Configuration::default();
let params = DeleteDmDomainParams {
    // parameters
};
delete_dm_domain(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_dm_domain

Show a domain

```rust
let cfg = &Configuration::default();
let params = GetDmDomainParams {
    // parameters
};
get_dm_domain(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

[**crate::models::SuccessfulResponseAsObject**](SuccessfulResponseAsObject.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_dm_domains

List all domains

```rust
let cfg = &Configuration::default();
let params = ListDmDomainsParams {
    // parameters
};
list_dm_domains(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fqdn** | Option\<**String**> |  |  |
**service_id** | Option\<**String**> | Filter results based on a service_id. |  |
**sort** | Option\<**String**> | The order in which to list the results. |  |[default to fqdn]
**activated** | Option\<**bool**> |  |  |
**verified** | Option\<**bool**> |  |  |
**cursor** | Option\<**String**> | Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty. |  |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 20]

### Return type

[**crate::models::InlineResponse2007**](InlineResponse2007.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_dm_domain

Update a domain

```rust
let cfg = &Configuration::default();
let params = UpdateDmDomainParams {
    // parameters
};
update_dm_domain(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |
**request_body_for_update** | Option\<[**RequestBodyForUpdate**](RequestBodyForUpdate.md)> |  |  |

### Return type

[**crate::models::SuccessfulResponseAsObject**](SuccessfulResponseAsObject.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

