# OperationGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**method** | **String** | The HTTP method for the operation. | 
**domain** | **String** | The domain for the operation. | 
**path** | **String** | The path for the operation, which may include path parameters. | 
**description** | Option<**String**> | A description of what the operation does. | 
**tag_ids** | Option<**Vec<String>**> | An array of operation tag IDs associated with this operation. | 
**id** | **String** | The unique identifier of the operation. | [readonly]
**created_at** | Option<**String**> | The timestamp when the operation was created. | [readonly]
**updated_at** | **String** | The timestamp when the operation was last updated. | [readonly]
**last_seen_at** | Option<**String**> | The timestamp when the operation was last seen in traffic. | [readonly]
**rps** | Option<**f32**> | Requests per second observed for this operation. | [readonly]
**status** | Option<**String**> | The status of the operation. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


