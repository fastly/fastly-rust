# WafFirewallVersionResponseDataAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**active** | Option<**bool**> | Whether a specific firewall version is currently deployed. | [readonly]
**active_rules_fastly_block_count** | Option<**i32**> | The number of active Fastly rules set to block. | [readonly]
**active_rules_fastly_log_count** | Option<**i32**> | The number of active Fastly rules set to log. | [readonly]
**active_rules_fastly_score_count** | Option<**i32**> | The number of active Fastly rules set to score. | [readonly]
**active_rules_owasp_block_count** | Option<**i32**> | The number of active OWASP rules set to block. | [readonly]
**active_rules_owasp_log_count** | Option<**i32**> | The number of active OWASP rules set to log. | [readonly]
**active_rules_owasp_score_count** | Option<**i32**> | The number of active OWASP rules set to score. | [readonly]
**active_rules_trustwave_block_count** | Option<**i32**> | The number of active Trustwave rules set to block. | [readonly]
**active_rules_trustwave_log_count** | Option<**i32**> | The number of active Trustwave rules set to log. | [readonly]
**last_deployment_status** | Option<**String**> | The status of the last deployment of this firewall version. | [readonly]
**deployed_at** | Option<**String**> | Time-stamp (GMT) indicating when the firewall version was last deployed. | [readonly]
**error** | Option<**String**> | Contains error message if the firewall version fails to deploy. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


