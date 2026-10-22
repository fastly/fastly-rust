# NetworkProtectNetworkUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Optional descriptive text for the network. This metadata does not change traffic handling. Null clears it on update. | 
**name** | Option<**String**> | A human-readable name, unique within the customer. | 
**rule_set_id** | Option<**String**> | Custom rule-set attachment is unavailable. Omission, null, and an empty string preserve the current backend-assigned rule set. | 
**endpoint_policies** | Option<[**Vec&lt;crate::models::NetworkProtectEndpointPolicy&gt;**](NetworkProtectEndpointPolicy.md)> | Complete endpoint policy configuration. A network has at most one policy per region and may have no policies. | 
**advertise** | Option<**bool**> | Desired advertisement state, which is not the same as observed route state. Enabling it requires approved LOA, valid ROA, and a ready default-region delivery policy. A later ROA failure does not turn an existing advertisement off; losing LOA approval or the last ready default endpoint does. | [default to false]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


