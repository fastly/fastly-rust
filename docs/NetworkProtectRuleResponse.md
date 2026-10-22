# NetworkProtectRuleResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ip_protocol** | [**crate::models::NetworkProtectRuleIpProtocol**](NetworkProtectRuleIpProtocol.md) |  | 
**source_prefix_list_id** | **String** | Match traffic whose source appears in this prefix list. | 
**destination_ip_address** | **String** | Match traffic sent to this address; omit for any destination. An address outside a Network using this Rule Set never matches and still consumes one of the 64 rules. The API does not enforce containment in a network prefix; attached Rule Sets require a compatible IP family. | 
**source_ports** | [**crate::models::NetworkProtectRuleSourcePorts**](NetworkProtectRuleSourcePorts.md) |  | 
**destination_ports** | [**crate::models::NetworkProtectRuleDestinationPorts**](NetworkProtectRuleDestinationPorts.md) |  | 
**created_at** | Option<**String**> | An ISO 8601 timestamp in UTC. | [readonly]
**updated_at** | Option<**String**> | An ISO 8601 timestamp in UTC. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


