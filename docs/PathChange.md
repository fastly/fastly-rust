# PathChange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path_id** | Option<**String**> | Alphanumeric string identifying the path. Stable across versions of the routing config. | [readonly]
**path** | Option<**String**> | The current path pattern. | 
**old_path** | Option<**String**> | The previous path pattern, if it changed. | 
**rules_added** | Option<[**Vec&lt;crate::models::RuleResponse&gt;**](RuleResponse.md)> | Rules that were added to this path. | 
**rules_changed** | Option<[**Vec&lt;crate::models::RuleChange&gt;**](RuleChange.md)> | Rules that were modified on this path. | 
**rules_deleted** | Option<[**Vec&lt;crate::models::RuleResponse&gt;**](RuleResponse.md)> | Rules that were removed from this path. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


