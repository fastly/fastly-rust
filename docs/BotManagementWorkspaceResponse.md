# BotManagementWorkspaceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The display name of the workspace. | 
**description** | Option<**String**> | A description of the workspace. | 
**protection_mode** | [**crate::models::BotManagementProtectionMode**](BotManagementProtectionMode.md) |  | 
**id** | **String** | A UUID used to uniquely identify the workspace. | [readonly]
**services** | Option<**Vec<String>**> | The IDs of the services attached to this workspace. | [readonly]
**created_at** | **String** | The date and time the workspace was created, in RFC 3339 format. | [readonly]
**updated_at** | **String** | The date and time the workspace was last updated, in RFC 3339 format. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


