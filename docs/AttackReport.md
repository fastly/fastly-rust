# AttackReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID of the workspace. | 
**name** | **String** | Name of the workspace. | 
**total_count** | **i32** | Total request count | 
**blocked_count** | **i32** | Blocked request count | 
**flagged_count** | **i32** | Flagged request count | 
**attack_count** | **i32** | Attack request count | 
**all_flagged_ip_count** | **i32** | Count of IPs that have been flagged | 
**flagged_ip_count** | **i32** | Count of currently flagged IPs | 
**top_attack_signals** | [**Vec&lt;crate::models::AttackSignal&gt;**](AttackSignal.md) |  | 
**top_attack_sources** | [**Vec&lt;crate::models::AttackSource&gt;**](AttackSource.md) |  | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


