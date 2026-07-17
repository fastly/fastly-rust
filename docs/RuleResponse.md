# RuleResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**id** | Option<**String**> | Alphanumeric string identifying the rule. Stable across versions of the routing config. | [readonly]
**is_default** | Option<**bool**> | Whether this is the default (catch-all) rule for the path. | [readonly]
**action** | Option<[**crate::models::Action**](Action.md)> |  | 
**conditions** | Option<[**Vec&lt;crate::models::RoutingConfigCondition&gt;**](RoutingConfigCondition.md)> | The conditions a request must satisfy for this rule to match. Empty for the default rule. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


