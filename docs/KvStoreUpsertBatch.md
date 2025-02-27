# KvStoreUpsertBatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | The key of the item to be added, updated, retrieved, or deleted. | 
**time_to_live_sec** | Option<**i32**> | Indicates that the item should be deleted after the specified number of seconds have elapsed. Deletion is not immediate but will occur within 24 hours of the requested expiration. | 
**metadata** | Option<**String**> | An arbitrary data field which can contain up to 2000 bytes of data. | 
**background_fetch** | Option<**bool**> | If set to true, the new value for the item will not be immediately visible to other users of the KV store; they will receive the existing (stale) value while the platform updates cached copies. Setting this to true ensures that other users of the KV store will receive responses to 'get' operations for this item quickly, although they will be slightly out of date. | [default to false]
**value** | **String** | Value for the item (in Base64 encoding). | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


