# DomainResearchApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**domain_status**](DomainResearchApi.md#domain_status) | **GET** /domain-management/v1/tools/status | Domain status
[**suggest_domains**](DomainResearchApi.md#suggest_domains) | **GET** /domain-management/v1/tools/suggest | Suggest domains



## domain_status

The `Status` method checks the availability status of a single domain name.

```rust
let cfg = &Configuration::default();
let params = DomainStatusParams {
    // parameters
};
domain_status(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**scope** | Option\<**String**> |  |  |

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## suggest_domains

The `Suggest` method performs a real-time query of the search term(s) against the [known zone database](http://zonedb.org), making recommendations, stemming, and applying Unicode folding, IDN normalization, registrar supported-zone restrictions, and other refinements. **Note:** `Suggest` method responses do not include domain availability status. 

```rust
let cfg = &Configuration::default();
let params = SuggestDomainsParams {
    // parameters
};
suggest_domains(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**defaults** | Option\<**String**> |  |  |
**keywords** | Option\<**String**> |  |  |
**location** | Option\<**String**> |  |  |
**vendor** | Option\<**String**> |  |  |

### Return type

[**crate::models::InlineResponse2006**](InlineResponse2006.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

