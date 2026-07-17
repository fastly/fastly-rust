# ProductBotManagementApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_bot_management**](ProductBotManagementApi.md#disable_product_bot_management) | **DELETE** /enabled-products/v1/bot_management/services/{service_id} | Disable product
[**enable_product_bot_management**](ProductBotManagementApi.md#enable_product_bot_management) | **PUT** /enabled-products/v1/bot_management/services/{service_id} | Enable product
[**get_product_bot_management**](ProductBotManagementApi.md#get_product_bot_management) | **GET** /enabled-products/v1/bot_management/services/{service_id} | Get product enablement status
[**get_product_bot_management_configuration**](ProductBotManagementApi.md#get_product_bot_management_configuration) | **GET** /enabled-products/v1/bot_management/services/{service_id}/configuration | Get configuration
[**get_services_product_bot_management**](ProductBotManagementApi.md#get_services_product_bot_management) | **GET** /enabled-products/v1/bot_management/services | Get services with product enabled
[**set_product_bot_management_configuration**](ProductBotManagementApi.md#set_product_bot_management_configuration) | **PATCH** /enabled-products/v1/bot_management/services/{service_id}/configuration | Update configuration



## disable_product_bot_management

Disable the Bot Management product on a service.

```rust
let cfg = &Configuration::default();
let params = DisableProductBotManagementParams {
    // parameters
};
disable_product_bot_management(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## enable_product_bot_management

Enable the Bot Management product on a service.

```rust
let cfg = &Configuration::default();
let params = EnableProductBotManagementParams {
    // parameters
};
enable_product_bot_management(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::BotManagementResponseBodyEnable**](BotManagementResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_bot_management

Get the enablement status of the Bot Management product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductBotManagementParams {
    // parameters
};
get_product_bot_management(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::BotManagementResponseBodyEnable**](BotManagementResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_bot_management_configuration

Get the configuration of the Bot Management product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductBotManagementConfigurationParams {
    // parameters
};
get_product_bot_management_configuration(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::BotManagementResponseConfigure**](BotManagementResponseConfigure.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_services_product_bot_management

Get all the services which have the Bot Management product enabled.

```rust
let cfg = &Configuration::default();
let params = GetServicesProductBotManagementParams {
    // parameters
};
get_services_product_bot_management(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BotManagementResponseBodyGetAllServices**](BotManagementResponseBodyGetAllServices.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## set_product_bot_management_configuration

Update the configuration of the Bot Management product on a service.

```rust
let cfg = &Configuration::default();
let params = SetProductBotManagementConfigurationParams {
    // parameters
};
set_product_bot_management_configuration(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**bot_management_request_update_configuration** | Option\<[**BotManagementRequestUpdateConfiguration**](BotManagementRequestUpdateConfiguration.md)> |  |  |

### Return type

[**crate::models::BotManagementResponseConfigure**](BotManagementResponseConfigure.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

