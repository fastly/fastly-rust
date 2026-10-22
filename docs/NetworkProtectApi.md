# NetworkProtectApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**create_network_protect_endpoint**](NetworkProtectApi.md#create_network_protect_endpoint) | **POST** /network-protect/v1/endpoints | Create a Network Protect endpoint
[**create_network_protect_network**](NetworkProtectApi.md#create_network_protect_network) | **POST** /network-protect/v1/networks | Create a Network Protect network
[**create_network_protect_prefix_list**](NetworkProtectApi.md#create_network_protect_prefix_list) | **POST** /network-protect/v1/prefix-lists | Create a Network Protect prefix list
[**create_network_protect_rule_set**](NetworkProtectApi.md#create_network_protect_rule_set) | **POST** /network-protect/v1/rule-sets | Create a Network Protect rule set
[**delete_network_protect_endpoint**](NetworkProtectApi.md#delete_network_protect_endpoint) | **DELETE** /network-protect/v1/endpoints/{endpoint_id} | Delete a Network Protect endpoint
[**delete_network_protect_network**](NetworkProtectApi.md#delete_network_protect_network) | **DELETE** /network-protect/v1/networks/{network_id} | Delete a Network Protect network
[**delete_network_protect_prefix_list**](NetworkProtectApi.md#delete_network_protect_prefix_list) | **DELETE** /network-protect/v1/prefix-lists/{prefix_list_id} | Delete a Network Protect prefix list
[**delete_network_protect_rule_set**](NetworkProtectApi.md#delete_network_protect_rule_set) | **DELETE** /network-protect/v1/rule-sets/{rule_set_id} | Delete a Network Protect rule set
[**get_network_protect_endpoint**](NetworkProtectApi.md#get_network_protect_endpoint) | **GET** /network-protect/v1/endpoints/{endpoint_id} | Get a Network Protect endpoint
[**get_network_protect_network**](NetworkProtectApi.md#get_network_protect_network) | **GET** /network-protect/v1/networks/{network_id} | Get a Network Protect network
[**get_network_protect_prefix_list**](NetworkProtectApi.md#get_network_protect_prefix_list) | **GET** /network-protect/v1/prefix-lists/{prefix_list_id} | Get a Network Protect prefix list
[**get_network_protect_rule_set**](NetworkProtectApi.md#get_network_protect_rule_set) | **GET** /network-protect/v1/rule-sets/{rule_set_id} | Get a Network Protect rule set
[**list_network_protect_endpoints**](NetworkProtectApi.md#list_network_protect_endpoints) | **GET** /network-protect/v1/endpoints | List Network Protect endpoints
[**list_network_protect_networks**](NetworkProtectApi.md#list_network_protect_networks) | **GET** /network-protect/v1/networks | List Network Protect networks
[**list_network_protect_prefix_lists**](NetworkProtectApi.md#list_network_protect_prefix_lists) | **GET** /network-protect/v1/prefix-lists | List Network Protect prefix lists
[**list_network_protect_rule_sets**](NetworkProtectApi.md#list_network_protect_rule_sets) | **GET** /network-protect/v1/rule-sets | List Network Protect rule sets
[**update_network_protect_endpoint**](NetworkProtectApi.md#update_network_protect_endpoint) | **PATCH** /network-protect/v1/endpoints/{endpoint_id} | Update a Network Protect endpoint
[**update_network_protect_network**](NetworkProtectApi.md#update_network_protect_network) | **PATCH** /network-protect/v1/networks/{network_id} | Update a Network Protect network
[**update_network_protect_prefix_list**](NetworkProtectApi.md#update_network_protect_prefix_list) | **PATCH** /network-protect/v1/prefix-lists/{prefix_list_id} | Update a Network Protect prefix list
[**update_network_protect_rule_set**](NetworkProtectApi.md#update_network_protect_rule_set) | **PATCH** /network-protect/v1/rule-sets/{rule_set_id} | Update a Network Protect rule set



## create_network_protect_endpoint

Creates a GRE delivery destination. A new endpoint is not attached to a network until it is added to an endpoint policy.

```rust
let cfg = &Configuration::default();
let params = CreateNetworkProtectEndpointParams {
    // parameters
};
create_network_protect_endpoint(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_protect_endpoint_create** | [**NetworkProtectEndpointCreate**](NetworkProtectEndpointCreate.md) |  | [required] |

### Return type

[**crate::models::NetworkProtectEndpointResponse**](NetworkProtectEndpointResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_network_protect_network

Registers customer-owned IP space for protection. The network starts with advertisement disabled and Fastly does not announce its prefix. Turn advertisement on after LOA and ROA authorization are ready. The create request cannot include advertise; sending it returns a `400 Bad Request` error.

```rust
let cfg = &Configuration::default();
let params = CreateNetworkProtectNetworkParams {
    // parameters
};
create_network_protect_network(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_protect_network_create** | [**NetworkProtectNetworkCreate**](NetworkProtectNetworkCreate.md) |  | [required] |

### Return type

[**crate::models::NetworkProtectNetworkResponse**](NetworkProtectNetworkResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_network_protect_prefix_list

Creates a typed list of addresses that rules can reference. The family is required and cannot be changed. The addresses list may remain empty.

```rust
let cfg = &Configuration::default();
let params = CreateNetworkProtectPrefixListParams {
    // parameters
};
create_network_protect_prefix_list(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_protect_prefix_list_create** | [**NetworkProtectPrefixListCreate**](NetworkProtectPrefixListCreate.md) |  | [required] |

### Return type

[**crate::models::NetworkProtectPrefixListResponse**](NetworkProtectPrefixListResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_network_protect_rule_set

Creates an ordered rule set and stores its filtering intent. A rule set has at most 64 rules and uses the default action when no rule matches. Custom attachment and detachment are restricted in this release.

```rust
let cfg = &Configuration::default();
let params = CreateNetworkProtectRuleSetParams {
    // parameters
};
create_network_protect_rule_set(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_protect_rule_set_create** | [**NetworkProtectRuleSetCreate**](NetworkProtectRuleSetCreate.md) |  | [required] |

### Return type

[**crate::models::NetworkProtectRuleSetResponse**](NetworkProtectRuleSetResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_network_protect_endpoint

Deletes an endpoint. The endpoint must first be removed from every network policy; active references return a `409 Conflict` error.

```rust
let cfg = &Configuration::default();
let params = DeleteNetworkProtectEndpointParams {
    // parameters
};
delete_network_protect_endpoint(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **String** | Unique identifier of the Network Protect endpoint. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_network_protect_network

Deletes a network and stops its advertisement and filtering. Endpoints and rule sets used by the network are not deleted.

```rust
let cfg = &Configuration::default();
let params = DeleteNetworkProtectNetworkParams {
    // parameters
};
delete_network_protect_network(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** | Unique identifier of the Network Protect network. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_network_protect_prefix_list

Deletes a prefix list. Active rule references must be removed first; referenced lists return a `409 Conflict` error. Unknown and foreign IDs are indistinguishable and return a `404 Not Found` error.

```rust
let cfg = &Configuration::default();
let params = DeleteNetworkProtectPrefixListParams {
    // parameters
};
delete_network_protect_prefix_list(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix_list_id** | **String** | Unique identifier of the Network Protect prefix list. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_network_protect_rule_set

Deletes a rule set. Attached rule sets cannot be deleted and return a `409 Conflict` error until their references are removed.

```rust
let cfg = &Configuration::default();
let params = DeleteNetworkProtectRuleSetParams {
    // parameters
};
delete_network_protect_rule_set(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_set_id** | **String** | Unique identifier of the Network Protect rule set. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_network_protect_endpoint

Retrieves a single endpoint by its identifier.

```rust
let cfg = &Configuration::default();
let params = GetNetworkProtectEndpointParams {
    // parameters
};
get_network_protect_endpoint(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **String** | Unique identifier of the Network Protect endpoint. | [required] |

### Return type

[**crate::models::NetworkProtectEndpointResponse**](NetworkProtectEndpointResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_network_protect_network

Retrieves a single network by its identifier.

```rust
let cfg = &Configuration::default();
let params = GetNetworkProtectNetworkParams {
    // parameters
};
get_network_protect_network(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** | Unique identifier of the Network Protect network. | [required] |

### Return type

[**crate::models::NetworkProtectNetworkResponse**](NetworkProtectNetworkResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_network_protect_prefix_list

Retrieves a single prefix list by its identifier. Unknown and foreign IDs are indistinguishable and return a `404 Not Found` error.

```rust
let cfg = &Configuration::default();
let params = GetNetworkProtectPrefixListParams {
    // parameters
};
get_network_protect_prefix_list(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix_list_id** | **String** | Unique identifier of the Network Protect prefix list. | [required] |

### Return type

[**crate::models::NetworkProtectPrefixListResponse**](NetworkProtectPrefixListResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_network_protect_rule_set

Retrieves a single rule set with rules in evaluation order.

```rust
let cfg = &Configuration::default();
let params = GetNetworkProtectRuleSetParams {
    // parameters
};
get_network_protect_rule_set(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_set_id** | **String** | Unique identifier of the Network Protect rule set. | [required] |

### Return type

[**crate::models::NetworkProtectRuleSetResponse**](NetworkProtectRuleSetResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_network_protect_endpoints

Lists the customer's GRE delivery endpoints in ascending ID order. The response is cursor-paginated and includes the effective page limit.

```rust
let cfg = &Configuration::default();
let params = ListNetworkProtectEndpointsParams {
    // parameters
};
list_network_protect_endpoints(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option\<**i64**> | Maximum resources to return. Defaults to 100. Positive values above 1000 are accepted and clamped to 1000; the effective value is reported in meta.limit. Zero, negative, and non-integer values return a `400 Bad Request` error. |  |[default to 100]
**cursor** | Option\<**String**> | Continuation value from a previous response's meta.next_cursor. Omit this parameter to request the first page. |  |

### Return type

[**crate::models::NetworkProtectEndpointsResponse**](NetworkProtectEndpointsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_network_protect_networks

Lists the customer's networks in ascending ID order. The response is cursor-paginated and includes the effective page limit.

```rust
let cfg = &Configuration::default();
let params = ListNetworkProtectNetworksParams {
    // parameters
};
list_network_protect_networks(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option\<**i64**> | Maximum resources to return. Defaults to 100. Positive values above 1000 are accepted and clamped to 1000; the effective value is reported in meta.limit. Zero, negative, and non-integer values return a `400 Bad Request` error. |  |[default to 100]
**cursor** | Option\<**String**> | Continuation value from a previous response's meta.next_cursor. Omit this parameter to request the first page. |  |

### Return type

[**crate::models::NetworkProtectNetworksResponse**](NetworkProtectNetworksResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_network_protect_prefix_lists

Lists prefix lists in ascending ID order. Positive limits above 1000 are accepted and clamped to 1000. Invalid limits return a `400 Bad Request` error.

```rust
let cfg = &Configuration::default();
let params = ListNetworkProtectPrefixListsParams {
    // parameters
};
list_network_protect_prefix_lists(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option\<**i64**> | Maximum resources to return. Defaults to 100. Positive values above 1000 are accepted and clamped to 1000; the effective value is reported in meta.limit. Zero, negative, and non-integer values return a `400 Bad Request` error. |  |[default to 100]
**cursor** | Option\<**String**> | Continuation value from a previous response's meta.next_cursor. Omit this parameter to request the first page. |  |

### Return type

[**crate::models::NetworkProtectPrefixListsResponse**](NetworkProtectPrefixListsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_network_protect_rule_sets

Lists rule sets in ascending ID order. Positive limits above 1000 are accepted and clamped to 1000; invalid limits return a `400 Bad Request` error.

```rust
let cfg = &Configuration::default();
let params = ListNetworkProtectRuleSetsParams {
    // parameters
};
list_network_protect_rule_sets(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option\<**i64**> | Maximum resources to return. Defaults to 100. Positive values above 1000 are accepted and clamped to 1000; the effective value is reported in meta.limit. Zero, negative, and non-integer values return a `400 Bad Request` error. |  |[default to 100]
**cursor** | Option\<**String**> | Continuation value from a previous response's meta.next_cursor. Omit this parameter to request the first page. |  |

### Return type

[**crate::models::NetworkProtectRuleSetsResponse**](NetworkProtectRuleSetsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_network_protect_endpoint

Updates an endpoint. Omitted fields remain unchanged. The IP address cannot be changed; create a new endpoint if the destination changes.

```rust
let cfg = &Configuration::default();
let params = UpdateNetworkProtectEndpointParams {
    // parameters
};
update_network_protect_endpoint(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **String** | Unique identifier of the Network Protect endpoint. | [required] |
**network_protect_endpoint_update** | [**NetworkProtectEndpointUpdate**](NetworkProtectEndpointUpdate.md) |  | [required] |

### Return type

[**crate::models::NetworkProtectEndpointResponse**](NetworkProtectEndpointResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_network_protect_network

Updates a network. Omitted fields remain unchanged. The prefix cannot be changed. Sending `endpoint_policies` replaces the complete policy list; an empty array removes every policy. Advertisement is desired state. Enabling it requires an approved LOA, a valid ROA, and a ready default-region delivery policy. A readiness failure returns a `409 Conflict` error.

```rust
let cfg = &Configuration::default();
let params = UpdateNetworkProtectNetworkParams {
    // parameters
};
update_network_protect_network(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** | Unique identifier of the Network Protect network. | [required] |
**network_protect_network_update** | [**NetworkProtectNetworkUpdate**](NetworkProtectNetworkUpdate.md) |  | [required] |

### Return type

[**crate::models::NetworkProtectNetworkResponse**](NetworkProtectNetworkResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_network_protect_prefix_list

Updates a prefix list. The family cannot be changed. Sending addresses replaces the complete list; an empty array clears it when no active source-list restriction applies. Addresses are canonicalized, deduplicated, and sorted by the backend.

```rust
let cfg = &Configuration::default();
let params = UpdateNetworkProtectPrefixListParams {
    // parameters
};
update_network_protect_prefix_list(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix_list_id** | **String** | Unique identifier of the Network Protect prefix list. | [required] |
**network_protect_prefix_list_update** | [**NetworkProtectPrefixListUpdate**](NetworkProtectPrefixListUpdate.md) |  | [required] |

### Return type

[**crate::models::NetworkProtectPrefixListResponse**](NetworkProtectPrefixListResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_network_protect_rule_set

Updates a rule set. Omitted fields remain unchanged. Sending rules replaces the complete ordered list; an empty array removes every rule. Custom attachment and detachment remain restricted in this release.

```rust
let cfg = &Configuration::default();
let params = UpdateNetworkProtectRuleSetParams {
    // parameters
};
update_network_protect_rule_set(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_set_id** | **String** | Unique identifier of the Network Protect rule set. | [required] |
**network_protect_rule_set_update** | [**NetworkProtectRuleSetUpdate**](NetworkProtectRuleSetUpdate.md) |  | [required] |

### Return type

[**crate::models::NetworkProtectRuleSetResponse**](NetworkProtectRuleSetResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

