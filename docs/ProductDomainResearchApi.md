# ProductDomainResearchApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_domain_research**](ProductDomainResearchApi.md#disable_product_domain_research) | **DELETE** /enabled-products/v1/domain_research | Disable product
[**enable_domain_research**](ProductDomainResearchApi.md#enable_domain_research) | **PUT** /enabled-products/v1/domain_research | Enable product
[**get_domain_research**](ProductDomainResearchApi.md#get_domain_research) | **GET** /enabled-products/v1/domain_research | Get product enablement status



## disable_product_domain_research

Disable the Domain Research product.

```rust
let cfg = &Configuration::default();
let params = DisableProductDomainResearchParams {
    // parameters
};
disable_product_domain_research(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## enable_domain_research

Enable the Domain Research product.

```rust
let cfg = &Configuration::default();
let params = EnableDomainResearchParams {
    // parameters
};
enable_domain_research(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DomainResearchResponseBodyEnable**](DomainResearchResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_domain_research

Get the enablement status of the Domain Research product.

```rust
let cfg = &Configuration::default();
let params = GetDomainResearchParams {
    // parameters
};
get_domain_research(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DomainResearchResponseBodyEnable**](DomainResearchResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

