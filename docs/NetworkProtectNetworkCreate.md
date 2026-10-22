# NetworkProtectNetworkCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Optional descriptive text for the network. This metadata does not change traffic handling. Null clears it on update. | 
**name** | **String** | A human-readable name, unique within the customer. | 
**prefix** | **String** | The CIDR block to protect, masked to its network address. It cannot be changed after creation. | 
**rule_set_id** | Option<**String**> | Custom rule-set attachment is unavailable. Omit this value or send an empty string to use the backend-assigned default rule set. | 
**endpoint_policies** | Option<[**Vec&lt;crate::models::NetworkProtectEndpointPolicy&gt;**](NetworkProtectEndpointPolicy.md)> | Complete endpoint policy configuration. A network has at most one policy per region and may have no policies. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


