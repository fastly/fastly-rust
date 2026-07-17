# DmRoutingConfigsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**activate_dm_routing_config_draft**](DmRoutingConfigsApi.md#activate_dm_routing_config_draft) | **POST** /domain-management/v1/routing-configs/{config_id}/activate | Activate the draft
[**create_dm_routing_config**](DmRoutingConfigsApi.md#create_dm_routing_config) | **POST** /domain-management/v1/routing-configs | Create a routing config
[**create_dm_routing_config_path**](DmRoutingConfigsApi.md#create_dm_routing_config_path) | **POST** /domain-management/v1/routing-configs/{config_id}/paths | Create a path
[**create_dm_routing_config_rule**](DmRoutingConfigsApi.md#create_dm_routing_config_rule) | **POST** /domain-management/v1/routing-configs/{config_id}/paths/{path_id}/rules | Create a rule
[**deactivate_dm_routing_config**](DmRoutingConfigsApi.md#deactivate_dm_routing_config) | **POST** /domain-management/v1/routing-configs/{config_id}/deactivate | Deactivate a routing config
[**delete_dm_routing_config**](DmRoutingConfigsApi.md#delete_dm_routing_config) | **DELETE** /domain-management/v1/routing-configs/{config_id} | Delete a routing config
[**delete_dm_routing_config_inactive_versions**](DmRoutingConfigsApi.md#delete_dm_routing_config_inactive_versions) | **DELETE** /domain-management/v1/routing-configs/{config_id}/versions/inactive | Delete inactive versions
[**delete_dm_routing_config_path**](DmRoutingConfigsApi.md#delete_dm_routing_config_path) | **DELETE** /domain-management/v1/routing-configs/{config_id}/paths/{path_id} | Delete a path
[**delete_dm_routing_config_rule**](DmRoutingConfigsApi.md#delete_dm_routing_config_rule) | **DELETE** /domain-management/v1/routing-configs/{config_id}/paths/{path_id}/rules/{rule_id} | Delete a rule
[**discard_dm_routing_config_draft**](DmRoutingConfigsApi.md#discard_dm_routing_config_draft) | **DELETE** /domain-management/v1/routing-configs/{config_id}/draft | Discard the draft
[**get_dm_routing_config**](DmRoutingConfigsApi.md#get_dm_routing_config) | **GET** /domain-management/v1/routing-configs/{config_id} | Get a routing config
[**get_dm_routing_config_draft_diff**](DmRoutingConfigsApi.md#get_dm_routing_config_draft_diff) | **GET** /domain-management/v1/routing-configs/{config_id}/draft/diff | Get the draft diff
[**get_dm_routing_config_path**](DmRoutingConfigsApi.md#get_dm_routing_config_path) | **GET** /domain-management/v1/routing-configs/{config_id}/paths/{path_id} | Get a path
[**get_dm_routing_config_rule**](DmRoutingConfigsApi.md#get_dm_routing_config_rule) | **GET** /domain-management/v1/routing-configs/{config_id}/paths/{path_id}/rules/{rule_id} | Get a rule
[**list_dm_routing_config_paths**](DmRoutingConfigsApi.md#list_dm_routing_config_paths) | **GET** /domain-management/v1/routing-configs/{config_id}/paths | List paths
[**list_dm_routing_config_rules**](DmRoutingConfigsApi.md#list_dm_routing_config_rules) | **GET** /domain-management/v1/routing-configs/{config_id}/paths/{path_id}/rules | List rules
[**list_dm_routing_config_versions**](DmRoutingConfigsApi.md#list_dm_routing_config_versions) | **GET** /domain-management/v1/routing-configs/{config_id}/versions | List versions
[**list_dm_routing_configs**](DmRoutingConfigsApi.md#list_dm_routing_configs) | **GET** /domain-management/v1/routing-configs | List routing configs
[**reactivate_dm_routing_config_version**](DmRoutingConfigsApi.md#reactivate_dm_routing_config_version) | **POST** /domain-management/v1/routing-configs/{config_id}/versions/{version_id}/activate | Reactivate a version
[**update_dm_routing_config_draft**](DmRoutingConfigsApi.md#update_dm_routing_config_draft) | **PATCH** /domain-management/v1/routing-configs/{config_id}/draft | Update the draft
[**update_dm_routing_config_path**](DmRoutingConfigsApi.md#update_dm_routing_config_path) | **PATCH** /domain-management/v1/routing-configs/{config_id}/paths/{path_id} | Update a path
[**update_dm_routing_config_rule**](DmRoutingConfigsApi.md#update_dm_routing_config_rule) | **PATCH** /domain-management/v1/routing-configs/{config_id}/paths/{path_id}/rules/{rule_id} | Update a rule



## activate_dm_routing_config_draft

Activate the current draft version. The previously active version, if any, becomes inactive but is retained in version history.

```rust
let cfg = &Configuration::default();
let params = ActivateDmRoutingConfigDraftParams {
    // parameters
};
activate_dm_routing_config_draft(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |

### Return type

[**crate::models::RoutingConfigVersionResponse**](RoutingConfigVersionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_dm_routing_config

Create a new routing config. An optional `initial_version` may be provided to seed the config with paths and rules in a single request, and may also be activated immediately.

```rust
let cfg = &Configuration::default();
let params = CreateDmRoutingConfigParams {
    // parameters
};
create_dm_routing_config(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**routing_config** | Option\<[**RoutingConfig**](RoutingConfig.md)> |  |  |

### Return type

[**crate::models::RoutingConfigResponse**](RoutingConfigResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_dm_routing_config_path

Add a new path to the config's draft version. If no draft exists, one is created automatically by cloning the active version.

```rust
let cfg = &Configuration::default();
let params = CreateDmRoutingConfigPathParams {
    // parameters
};
create_dm_routing_config_path(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**path_create** | Option\<[**PathCreate**](PathCreate.md)> |  |  |

### Return type

[**crate::models::PathResponse**](PathResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_dm_routing_config_rule

Add a new rule to a path on the config's draft version. If no draft exists, one is created automatically by cloning the active version. A rule with an empty `conditions` array is a default (catch-all) rule and there can be at most one default rule per path.

```rust
let cfg = &Configuration::default();
let params = CreateDmRoutingConfigRuleParams {
    // parameters
};
create_dm_routing_config_rule(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**path_id** | **String** |  | [required] |
**rule_create** | Option\<[**RuleCreate**](RuleCreate.md)> |  |  |

### Return type

[**crate::models::RuleResponse**](RuleResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## deactivate_dm_routing_config

Clear the active version designation. This is a bookkeeping operation only — it does not stop edge traffic. Minerva continues serving the last-activated version until the domain association is removed in Spotless. Only removing the routing config from the domain (via Spotless) triggers Neptune to drop the reference, which causes Minerva to stop fetching and eventually clean up the cached config. Idempotent: returns 200 even if already deactivated.

```rust
let cfg = &Configuration::default();
let params = DeactivateDmRoutingConfigParams {
    // parameters
};
deactivate_dm_routing_config(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |

### Return type

[**crate::models::RoutingConfigResponse**](RoutingConfigResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_dm_routing_config

Delete a routing config. By default, configs that have an active version cannot be deleted. Pass `force=true` to bypass the active-version check — this is destructive and will immediately stop traffic routing for any paths the config serves. The `force` parameter does **not** bypass the domain-association check; if domains are still associated, deletion is rejected with 409 regardless of `force`.

```rust
let cfg = &Configuration::default();
let params = DeleteDmRoutingConfigParams {
    // parameters
};
delete_dm_routing_config(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**force** | Option\<**bool**> | When `true`, allows deleting a routing config that has an active version. This is destructive — traffic routing for any paths served by the config will stop immediately. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_dm_routing_config_inactive_versions

Delete all inactive versions for a routing config. The currently active version, if any, is retained.

```rust
let cfg = &Configuration::default();
let params = DeleteDmRoutingConfigInactiveVersionsParams {
    // parameters
};
delete_dm_routing_config_inactive_versions(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_dm_routing_config_path

Delete a path from the config's draft version. If no draft exists, one is created automatically by cloning the active version.

```rust
let cfg = &Configuration::default();
let params = DeleteDmRoutingConfigPathParams {
    // parameters
};
delete_dm_routing_config_path(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**path_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_dm_routing_config_rule

Delete a rule from the config's draft version. If no draft exists, one is created automatically by cloning the active version.

```rust
let cfg = &Configuration::default();
let params = DeleteDmRoutingConfigRuleParams {
    // parameters
};
delete_dm_routing_config_rule(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**path_id** | **String** |  | [required] |
**rule_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## discard_dm_routing_config_draft

Delete the current draft version, reverting any unactivated changes.

```rust
let cfg = &Configuration::default();
let params = DiscardDmRoutingConfigDraftParams {
    // parameters
};
discard_dm_routing_config_draft(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_dm_routing_config

Retrieve a single routing config by its identifier.

```rust
let cfg = &Configuration::default();
let params = GetDmRoutingConfigParams {
    // parameters
};
get_dm_routing_config(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |

### Return type

[**crate::models::RoutingConfigResponse**](RoutingConfigResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_dm_routing_config_draft_diff

Compare the current draft version against the active version and return the paths and rules that have been added, modified, or deleted.

```rust
let cfg = &Configuration::default();
let params = GetDmRoutingConfigDraftDiffParams {
    // parameters
};
get_dm_routing_config_draft_diff(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |

### Return type

[**crate::models::DraftDiff**](DraftDiff.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_dm_routing_config_path

Retrieve a single path by its stable identifier.

```rust
let cfg = &Configuration::default();
let params = GetDmRoutingConfigPathParams {
    // parameters
};
get_dm_routing_config_path(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**path_id** | **String** |  | [required] |

### Return type

[**crate::models::PathResponse**](PathResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_dm_routing_config_rule

Retrieve a single rule by its stable identifier.

```rust
let cfg = &Configuration::default();
let params = GetDmRoutingConfigRuleParams {
    // parameters
};
get_dm_routing_config_rule(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**path_id** | **String** |  | [required] |
**rule_id** | **String** |  | [required] |

### Return type

[**crate::models::RuleResponse**](RuleResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_dm_routing_config_paths

List paths for the config. Returns paths from the active version if one exists, otherwise from the draft.

```rust
let cfg = &Configuration::default();
let params = ListDmRoutingConfigPathsParams {
    // parameters
};
list_dm_routing_config_paths(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**path** | Option\<**String**> | Filter results by path pattern. The match strategy is controlled by the `match` parameter. |  |
**_match** | Option\<**String**> | How to match the value of the `path` filter against existing path patterns. Has no effect unless `path` is also provided. |  |[default to exact]
**sort** | Option\<**String**> | The order in which to list the results. |  |[default to -created_at]
**cursor** | Option\<**String**> | Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty. |  |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 20]

### Return type

[**crate::models::PathsResponse**](PathsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_dm_routing_config_rules

List all rules for a path in evaluation order.

```rust
let cfg = &Configuration::default();
let params = ListDmRoutingConfigRulesParams {
    // parameters
};
list_dm_routing_config_rules(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**path_id** | **String** |  | [required] |
**sort** | Option\<**String**> | The order in which to list the results. |  |[default to position]
**cursor** | Option\<**String**> | Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty. |  |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 20]

### Return type

[**crate::models::RulesResponse**](RulesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_dm_routing_config_versions

List all versions for a routing config.

```rust
let cfg = &Configuration::default();
let params = ListDmRoutingConfigVersionsParams {
    // parameters
};
list_dm_routing_config_versions(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**sort** | Option\<**String**> | The order in which to list the results. |  |[default to -activated_at]
**cursor** | Option\<**String**> | Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty. |  |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 20]

### Return type

[**crate::models::VersionsResponse**](VersionsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_dm_routing_configs

List all routing configs for the authenticated customer.

```rust
let cfg = &Configuration::default();
let params = ListDmRoutingConfigsParams {
    // parameters
};
list_dm_routing_configs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state** | Option\<[**Vec&lt;String&gt;**](String.md)> | Filter configs by lifecycle state. Accepts a comma-separated list of state values (e.g. `?state=active,active-with-draft`). Returns only configs whose current state matches one of the provided values. Returns 400 if any value is not a recognised state. |  |
**sort** | Option\<**String**> | The order in which to list the results. |  |[default to -created_at]
**cursor** | Option\<**String**> | Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty. |  |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 20]

### Return type

[**crate::models::RoutingConfigsResponse**](RoutingConfigsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## reactivate_dm_routing_config_version

Reactivate a previously-active version. The currently active version, if any, becomes inactive but is retained in version history.

```rust
let cfg = &Configuration::default();
let params = ReactivateDmRoutingConfigVersionParams {
    // parameters
};
reactivate_dm_routing_config_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

[**crate::models::RoutingConfigVersionResponse**](RoutingConfigVersionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_dm_routing_config_draft

Update metadata on the draft version, such as its comment. If no draft exists, one is created automatically by cloning the active version.

```rust
let cfg = &Configuration::default();
let params = UpdateDmRoutingConfigDraftParams {
    // parameters
};
update_dm_routing_config_draft(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**draft_update** | Option\<[**DraftUpdate**](DraftUpdate.md)> |  |  |

### Return type

[**crate::models::RoutingConfigVersionResponse**](RoutingConfigVersionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_dm_routing_config_path

Update a path on the config's draft version. If no draft exists, one is created automatically by cloning the active version.

```rust
let cfg = &Configuration::default();
let params = UpdateDmRoutingConfigPathParams {
    // parameters
};
update_dm_routing_config_path(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**path_id** | **String** |  | [required] |
**path_update** | Option\<[**PathUpdate**](PathUpdate.md)> |  |  |

### Return type

[**crate::models::PathResponse**](PathResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_dm_routing_config_rule

Update a rule on the config's draft version. If no draft exists, one is created automatically by cloning the active version.

```rust
let cfg = &Configuration::default();
let params = UpdateDmRoutingConfigRuleParams {
    // parameters
};
update_dm_routing_config_rule(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_id** | **String** |  | [required] |
**path_id** | **String** |  | [required] |
**rule_id** | **String** |  | [required] |
**rule_update** | Option\<[**RuleUpdate**](RuleUpdate.md)> |  |  |

### Return type

[**crate::models::RuleResponse**](RuleResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

