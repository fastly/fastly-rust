# ClientSideProtectionApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**csp_create_page**](ClientSideProtectionApi.md#csp_create_page) | **POST** /client-side-protection/v1/pages | Create page
[**csp_create_policy**](ClientSideProtectionApi.md#csp_create_policy) | **POST** /client-side-protection/v1/pages/{page_id}/policies | Create policy
[**csp_create_website**](ClientSideProtectionApi.md#csp_create_website) | **POST** /client-side-protection/v1/websites | Create website
[**csp_delete_page**](ClientSideProtectionApi.md#csp_delete_page) | **DELETE** /client-side-protection/v1/pages/{page_id} | Delete page
[**csp_delete_website**](ClientSideProtectionApi.md#csp_delete_website) | **DELETE** /client-side-protection/v1/websites/{website_id} | Delete website
[**csp_get_page**](ClientSideProtectionApi.md#csp_get_page) | **GET** /client-side-protection/v1/pages/{page_id} | Get page
[**csp_get_policy**](ClientSideProtectionApi.md#csp_get_policy) | **GET** /client-side-protection/v1/pages/{page_id}/policies/{policy_id} | Get policy
[**csp_get_script**](ClientSideProtectionApi.md#csp_get_script) | **GET** /client-side-protection/v1/pages/{page_id}/scripts/{script_id} | Get script
[**csp_get_website**](ClientSideProtectionApi.md#csp_get_website) | **GET** /client-side-protection/v1/websites/{website_id} | Get website
[**csp_list_header_events**](ClientSideProtectionApi.md#csp_list_header_events) | **GET** /client-side-protection/v1/pages/{page_id}/events | List header events
[**csp_list_headers**](ClientSideProtectionApi.md#csp_list_headers) | **GET** /client-side-protection/v1/pages/{page_id}/headers | List security headers
[**csp_list_pages**](ClientSideProtectionApi.md#csp_list_pages) | **GET** /client-side-protection/v1/pages | List pages
[**csp_list_policies**](ClientSideProtectionApi.md#csp_list_policies) | **GET** /client-side-protection/v1/pages/{page_id}/policies | List policies
[**csp_list_policy_reports**](ClientSideProtectionApi.md#csp_list_policy_reports) | **GET** /client-side-protection/v1/pages/{page_id}/policies/{policy_id}/reports | List policy reports
[**csp_list_scripts**](ClientSideProtectionApi.md#csp_list_scripts) | **GET** /client-side-protection/v1/pages/{page_id}/scripts | List scripts
[**csp_list_websites**](ClientSideProtectionApi.md#csp_list_websites) | **GET** /client-side-protection/v1/websites | List websites
[**csp_update_page**](ClientSideProtectionApi.md#csp_update_page) | **PATCH** /client-side-protection/v1/pages/{page_id} | Update page
[**csp_update_policy**](ClientSideProtectionApi.md#csp_update_policy) | **PATCH** /client-side-protection/v1/pages/{page_id}/policies/{policy_id} | Update policy
[**csp_update_script**](ClientSideProtectionApi.md#csp_update_script) | **PATCH** /client-side-protection/v1/pages/{page_id}/scripts/{script_id} | Update script
[**csp_update_website**](ClientSideProtectionApi.md#csp_update_website) | **PATCH** /client-side-protection/v1/websites/{website_id} | Update website



## csp_create_page

Create a new page for monitoring.

```rust
let cfg = &Configuration::default();
let params = CspCreatePageParams {
    // parameters
};
csp_create_page(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_create** | Option\<[**PageCreate**](PageCreate.md)> |  |  |

### Return type

[**crate::models::Page**](Page.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_create_policy

Create a new Content Security Policy for a page.

```rust
let cfg = &Configuration::default();
let params = CspCreatePolicyParams {
    // parameters
};
csp_create_policy(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |
**policy_create** | Option\<[**PolicyCreate**](PolicyCreate.md)> |  |  |

### Return type

[**crate::models::Policy**](Policy.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_create_website

Create a new website for Client-Side Protection monitoring.

```rust
let cfg = &Configuration::default();
let params = CspCreateWebsiteParams {
    // parameters
};
csp_create_website(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**website_create** | Option\<[**WebsiteCreate**](WebsiteCreate.md)> |  |  |

### Return type

[**crate::models::Website**](Website.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_delete_page

Delete a page and all associated scripts and policies.

```rust
let cfg = &Configuration::default();
let params = CspDeletePageParams {
    // parameters
};
csp_delete_page(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_delete_website

Delete a website and all associated pages, scripts, and policies.

```rust
let cfg = &Configuration::default();
let params = CspDeleteWebsiteParams {
    // parameters
};
csp_delete_website(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**website_id** | **String** | Website identifier | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_get_page

Get details for a specific page.

```rust
let cfg = &Configuration::default();
let params = CspGetPageParams {
    // parameters
};
csp_get_page(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |

### Return type

[**crate::models::Page**](Page.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_get_policy

Get details for a specific policy.

```rust
let cfg = &Configuration::default();
let params = CspGetPolicyParams {
    // parameters
};
csp_get_policy(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |
**policy_id** | **String** | Policy identifier | [required] |

### Return type

[**crate::models::Policy**](Policy.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_get_script

Get details for a specific script.

```rust
let cfg = &Configuration::default();
let params = CspGetScriptParams {
    // parameters
};
csp_get_script(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |
**script_id** | **String** | Script identifier | [required] |

### Return type

[**crate::models::Script**](Script.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_get_website

Get details for a specific website.

```rust
let cfg = &Configuration::default();
let params = CspGetWebsiteParams {
    // parameters
};
csp_get_website(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**website_id** | **String** | Website identifier | [required] |

### Return type

[**crate::models::Website**](Website.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_list_header_events

List security header change events for a page.

```rust
let cfg = &Configuration::default();
let params = CspListHeaderEventsParams {
    // parameters
};
csp_list_header_events(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 100]
**page** | Option\<**i32**> | Page number of the collection to request. |  |[default to 0]

### Return type

[**crate::models::InlineResponse20011**](InlineResponse20011.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_list_headers

List security headers detected on a page.

```rust
let cfg = &Configuration::default();
let params = CspListHeadersParams {
    // parameters
};
csp_list_headers(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 100]
**page** | Option\<**i32**> | Page number of the collection to request. |  |[default to 0]

### Return type

[**crate::models::InlineResponse20010**](InlineResponse20010.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_list_pages

List all pages. Optionally filter by website.

```rust
let cfg = &Configuration::default();
let params = CspListPagesParams {
    // parameters
};
csp_list_pages(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**website_id** | Option\<**String**> | Filter pages by website ID |  |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 100]
**page** | Option\<**i32**> | Page number of the collection to request. |  |[default to 0]

### Return type

[**crate::models::InlineResponse2006**](InlineResponse2006.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_list_policies

List all Content Security Policies for a page.

```rust
let cfg = &Configuration::default();
let params = CspListPoliciesParams {
    // parameters
};
csp_list_policies(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 100]
**page** | Option\<**i32**> | Page number of the collection to request. |  |[default to 0]

### Return type

[**crate::models::InlineResponse2008**](InlineResponse2008.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_list_policy_reports

List CSP violation reports for a policy.

```rust
let cfg = &Configuration::default();
let params = CspListPolicyReportsParams {
    // parameters
};
csp_list_policy_reports(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |
**policy_id** | **String** | Policy identifier | [required] |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 100]
**page** | Option\<**i32**> | Page number of the collection to request. |  |[default to 0]

### Return type

[**crate::models::InlineResponse2009**](InlineResponse2009.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_list_scripts

List all scripts detected on a page.

```rust
let cfg = &Configuration::default();
let params = CspListScriptsParams {
    // parameters
};
csp_list_scripts(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 100]
**page** | Option\<**i32**> | Page number of the collection to request. |  |[default to 0]

### Return type

[**crate::models::InlineResponse2007**](InlineResponse2007.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_list_websites

List all websites configured for Client-Side Protection.

```rust
let cfg = &Configuration::default();
let params = CspListWebsitesParams {
    // parameters
};
csp_list_websites(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 100]
**page** | Option\<**i32**> | Page number of the collection to request. |  |[default to 0]

### Return type

[**crate::models::InlineResponse2005**](InlineResponse2005.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_update_page

Update a page's configuration.

```rust
let cfg = &Configuration::default();
let params = CspUpdatePageParams {
    // parameters
};
csp_update_page(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |
**page_update** | Option\<[**PageUpdate**](PageUpdate.md)> |  |  |

### Return type

[**crate::models::Page**](Page.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_update_policy

Update a policy's configuration.

```rust
let cfg = &Configuration::default();
let params = CspUpdatePolicyParams {
    // parameters
};
csp_update_policy(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |
**policy_id** | **String** | Policy identifier | [required] |
**policy_update** | Option\<[**PolicyUpdate**](PolicyUpdate.md)> |  |  |

### Return type

[**crate::models::Policy**](Policy.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_update_script

Update a script's authorization status or justification.

```rust
let cfg = &Configuration::default();
let params = CspUpdateScriptParams {
    // parameters
};
csp_update_script(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_id** | **String** | Page identifier | [required] |
**script_id** | **String** | Script identifier | [required] |
**script_update** | Option\<[**ScriptUpdate**](ScriptUpdate.md)> |  |  |

### Return type

[**crate::models::Script**](Script.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## csp_update_website

Update a website's configuration.

```rust
let cfg = &Configuration::default();
let params = CspUpdateWebsiteParams {
    // parameters
};
csp_update_website(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**website_id** | **String** | Website identifier | [required] |
**website_update** | Option\<[**WebsiteUpdate**](WebsiteUpdate.md)> |  |  |

### Return type

[**crate::models::Website**](Website.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

