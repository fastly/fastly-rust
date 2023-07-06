# LegacyWafFirewall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_push** | Option<**String**> | Date and time that VCL was last pushed to cache nodes. | 
**prefetch_condition** | Option<**String**> | Name of the corresponding condition object. | 
**response** | Option<**String**> | Name of the corresponding response object. | 
**version** | Option<[**crate::models::ReadOnlyVersion**](ReadOnlyVersion.md)> |  | 
**service_id** | Option<[**crate::models::ReadOnlyServiceId**](ReadOnlyServiceId.md)> |  | 
**disabled** | Option<**bool**> | The status of the firewall. | [default to false]
**rule_statuses_log_count** | Option<**i32**> | The number of rule statuses set to log. | 
**rule_statuses_block_count** | Option<**i32**> | The number of rule statuses set to block. | 
**rule_statuses_disabled_count** | Option<**i32**> | The number of rule statuses set to disabled. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


