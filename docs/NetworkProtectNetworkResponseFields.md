# NetworkProtectNetworkResponseFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A read-only unique identifier for a Network Protect resource. | [readonly]
**roa_status** | Option<[**crate::models::NetworkProtectRoaStatus**](NetworkProtectRoaStatus.md)> |  | 
**roa_checked_at** | Option<**String**> | When Fastly last completed a ROA check. Null while the status is pending. A failed check leaves this unchanged; an old value can indicate delayed or failing checks. Null also means the controller did not report a check time. | [readonly]
**roa_data_as_of** | Option<**String**> | When the RPKI dataset used by the last check was built. This is not the same as `roa_checked_at`; a recent check can use an older dataset. Null while the status is pending or when no dataset build time was reported. | [readonly]
**loa_status** | Option<[**crate::models::NetworkProtectLoaStatus**](NetworkProtectLoaStatus.md)> |  | 
**created_at** | **String** | An ISO 8601 timestamp in UTC. | [readonly]
**updated_at** | **String** | An ISO 8601 timestamp in UTC. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


